from __future__ import annotations

import importlib.util
import json
import tempfile
import unittest
from pathlib import Path


SCRIPT = (
    Path(__file__).resolve().parents[1] / "scripts" / "validate_guidance.py"
)
SPEC = importlib.util.spec_from_file_location("validate_guidance", SCRIPT)
assert SPEC and SPEC.loader
validator = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(validator)


class AgentYamlTests(unittest.TestCase):
    def parse(self, text: str) -> tuple[dict[str, str | bool], list[str]]:
        errors: list[str] = []
        values = validator.parse_agent_yaml(text, Path("openai.yaml"), errors)
        return values, errors

    def test_accepts_strict_interface_shape(self) -> None:
        values, errors = self.parse(
            'interface:\n'
            '  display_name: "Example Skill"\n'
            '  short_description: "Describe this example skill"\n'
            '  default_prompt: "Use $example-skill to do the work."\n'
            'policy:\n'
            '  allow_implicit_invocation: false\n'
        )
        self.assertFalse(errors)
        self.assertEqual(values["display_name"], "Example Skill")
        self.assertIs(values["allow_implicit_invocation"], False)

    def test_rejects_wrong_nesting_and_duplicate_keys(self) -> None:
        _, errors = self.parse(
            'wrong_root:\n'
            '  display_name: "Example"\n'
            '  display_name: "Duplicate"\n'
        )
        self.assertTrue(errors)

    def test_rejects_malformed_or_extra_fields(self) -> None:
        _, errors = self.parse(
            'interface:\n'
            ' display_name: "Bad indentation"\n'
            '  short_description: not-quoted\n'
            '  default_prompt: "Use $example-skill."\n'
            '  extra: "Not allowed"\n'
        )
        self.assertTrue(errors)

    def test_rejects_blank_display_name(self) -> None:
        _, errors = self.parse(
            'interface:\n'
            '  display_name: ""\n'
            '  short_description: "Describe this example skill"\n'
            '  default_prompt: "Use $example-skill to do the work."\n'
        )
        self.assertTrue(errors)

    def test_requires_explicit_only_policy(self) -> None:
        _, missing_errors = self.parse(
            'interface:\n'
            '  display_name: "Example Skill"\n'
            '  short_description: "Describe this example skill"\n'
            '  default_prompt: "Use $example-skill to do the work."\n'
        )
        self.assertTrue(
            any("missing policy fields" in error for error in missing_errors)
        )

        _, implicit_errors = self.parse(
            'interface:\n'
            '  display_name: "Example Skill"\n'
            '  short_description: "Describe this example skill"\n'
            '  default_prompt: "Use $example-skill to do the work."\n'
            'policy:\n'
            '  allow_implicit_invocation: true\n'
        )
        self.assertTrue(
            any("must be false" in error for error in implicit_errors)
        )


class LinkValidationTests(unittest.TestCase):
    def validate(self, markdown: str, create: tuple[str, ...] = ()) -> list[str]:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            document = root / "README.md"
            document.write_text(markdown, encoding="utf-8")
            for relative in create:
                target = root / relative
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text("fixture", encoding="utf-8")
            errors: list[str] = []
            validator.validate_local_links(errors, root, [document])
            return errors

    def test_accepts_titles_angle_paths_and_images(self) -> None:
        errors = self.validate(
            '[doc](docs/a.md "A title")\n'
            '[quoted](docs/a.md "A ) title")\n'
            '[space](<docs/with space.md> "Space title")\n'
            '[paren](docs/a.md (Parenthesized title))\n'
            '[doc \\[draft\\]](docs/a.md)\n'
            '![image](docs/image.png)\n',
            ("docs/a.md", "docs/with space.md", "docs/image.png"),
        )
        self.assertFalse(errors)

    def test_accepts_angle_destination_with_closing_parenthesis(self) -> None:
        errors = self.validate(
            '[angle](<docs/b)c.md>)\n',
            ("docs/b)c.md",),
        )
        self.assertFalse(errors)

    def test_ignores_escaped_link_opener(self) -> None:
        errors = self.validate(r"Literal \[not a link](plain words)" + "\n")
        self.assertFalse(errors)

    def test_raw_containment_catches_nested_image_escape(self) -> None:
        errors = self.validate(
            "[![moon](../../outside.png)](docs/a.md)\n",
            ("docs/a.md",),
        )
        self.assertTrue(any("parent traversal" in error for error in errors))

    def test_raw_containment_catches_unmatched_backtick_escape(self) -> None:
        errors = self.validate(
            "literal ` then [outside](../../outside.md)\n"
            r"literal \` then [outside](../../outside.md)" + "\n"
        )
        self.assertEqual(sum("parent traversal" in error for error in errors), 2)

    def test_raw_containment_catches_invalid_fence_escape(self) -> None:
        errors = self.validate(
            "``` bad`info\n"
            "[outside](../../outside.md)\n"
        )
        self.assertTrue(any("parent traversal" in error for error in errors))

    def test_raw_containment_decodes_percent_and_html_escapes(self) -> None:
        errors = self.validate(
            "[percent](%2e%2e/outside.md)\n"
            "[entity](&#46;&#46;/outside.md)\n"
        )
        self.assertEqual(sum("parent traversal" in error for error in errors), 2)

    def test_raw_containment_catches_multiline_absolute_target(self) -> None:
        errors = self.validate(
            "` literal [outside](\n"
            "/outside.md)\n"
        )
        self.assertTrue(
            any("line-leading filesystem destination" in error for error in errors)
        )

    def test_accepts_reference_style_link_and_validates_target(self) -> None:
        errors = self.validate(
            '[project][project-doc]\n'
            '[project-doc]: docs/a.md "Project"\n',
            ("docs/a.md",),
        )
        self.assertFalse(errors)

    def test_rejects_root_relative_and_missing_links(self) -> None:
        errors = self.validate("[root](/missing.md)\n[missing](docs/missing.md)\n")
        self.assertEqual(len(errors), 2)

    def test_rejects_repository_escape_even_when_target_exists(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary)
            root = parent / "repo"
            root.mkdir()
            outside = parent / "outside.md"
            outside.write_text("outside", encoding="utf-8")
            document = root / "README.md"
            document.write_text(
                "[doc \\[draft\\]](../outside.md)\n", encoding="utf-8"
            )
            errors: list[str] = []
            validator.validate_local_links(errors, root, [document])
            self.assertEqual(len(errors), 1)
            self.assertIn("parent traversal", errors[0])

    def test_rejects_reference_style_repository_escape(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary)
            root = parent / "repo"
            root.mkdir()
            (parent / "outside.md").write_text("outside", encoding="utf-8")
            document = root / "README.md"
            document.write_text(
                '[doc \\[draft\\]][outside]\n[outside]: ../outside.md\n',
                encoding="utf-8",
            )
            errors: list[str] = []
            validator.validate_local_links(errors, root, [document])
            self.assertTrue(any("parent traversal" in error for error in errors))

    def test_rejects_blockquote_and_list_reference_escapes(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary)
            root = parent / "repo"
            root.mkdir()
            (parent / "outside.md").write_text("outside", encoding="utf-8")
            document = root / "README.md"
            document.write_text(
                "[quote][quote-ref]\n"
                "> [quote-ref]: ../outside.md\n"
                "[list][list-ref]\n"
                "- [list-ref]: ../outside.md\n",
                encoding="utf-8",
            )
            errors: list[str] = []
            validator.validate_local_links(errors, root, [document])
            self.assertEqual(
                sum("parent traversal" in error for error in errors), 2
            )

    def test_fence_length_and_indentation_cannot_hide_escape(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            parent = Path(temporary)
            root = parent / "repo"
            root.mkdir()
            (parent / "outside.md").write_text("outside", encoding="utf-8")
            document = root / "README.md"
            document.write_text(
                "````\n"
                "code\n"
                "```\n"
                "````\n"
                "[after-long-fence](../outside.md)\n"
                "    ```\n"
                "[after-indented-pseudo-fence](../outside.md)\n",
                encoding="utf-8",
            )
            errors: list[str] = []
            validator.validate_local_links(errors, root, [document])
            self.assertEqual(
                sum("parent traversal" in error for error in errors), 2
            )

    def test_rejects_missing_reference_definition(self) -> None:
        errors = self.validate("[project][missing]\n")
        self.assertEqual(len(errors), 1)

    def test_rejects_windows_unc_and_file_urls(self) -> None:
        errors = self.validate(
            '[drive](C:\\secret.txt)\n'
            '[unc](\\\\server\\share\\secret.txt)\n'
            '[file](file:///secret.txt)\n'
        )
        self.assertEqual(len(errors), 3)


class ManifestTests(unittest.TestCase):
    def write_manifest(self, root: Path, value: object) -> None:
        manifest = root / ".codex-plugin" / "plugin.json"
        manifest.parent.mkdir(parents=True)
        manifest.write_text(json.dumps(value), encoding="utf-8")

    def test_rejects_non_object_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "plugin"
            self.write_manifest(root, [])
            errors: list[str] = []
            validator.validate_manifest(errors, root)
            self.assertTrue(errors)

    def test_rejects_non_object_interface(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary) / "plugin"
            self.write_manifest(
                root,
                {
                    "name": "plugin",
                    "version": "1.0.0",
                    "skills": "./skills/",
                    "interface": [],
                },
            )
            errors: list[str] = []
            validator.validate_manifest(errors, root)
            self.assertTrue(errors)


class SkillBundleTests(unittest.TestCase):
    def test_reports_unreadable_agent_metadata(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            skill_root = Path(temporary) / "skills" / "refresh-product-guidance"
            skill_root.mkdir(parents=True)
            (skill_root / "SKILL.md").write_text(
                "---\n"
                "name: refresh-product-guidance\n"
                "description: Keep project guidance aligned.\n"
                "---\n\n"
                "# Refresh Product Guidance\n",
                encoding="utf-8",
            )
            agent_path = skill_root / "agents" / "openai.yaml"
            agent_path.parent.mkdir()
            agent_path.write_bytes(b"\xff")

            errors: list[str] = []
            count = validator.validate_skills(errors, skill_root.parent)

            self.assertEqual(count, 1)
            self.assertTrue(
                any("cannot read agent metadata" in error for error in errors)
            )

    def test_requires_manual_fallback_skill(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            skills_root = Path(temporary) / "skills"
            skills_root.mkdir()
            errors: list[str] = []
            count = validator.validate_skills(errors, skills_root)
            self.assertEqual(count, 0)
            self.assertTrue(any("refresh-product-guidance" in error for error in errors))


if __name__ == "__main__":
    unittest.main()
