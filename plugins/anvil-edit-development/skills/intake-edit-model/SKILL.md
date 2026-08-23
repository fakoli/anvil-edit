---
name: intake-edit-model
description: Assess a predictive-editing model artifact for Anvil Edit discovery and benchmark admission. Use for model pulses, artifact/license checks, native protocol mapping, runtime compatibility hypotheses, or choosing benchmark, spike, watch, or reject actions.
---

# Intake Edit Model

## Workflow

1. Start from primary sources: official model repository/card, immutable
   revision, license, tokenizer, protocol/template, runtime documentation, and
   research paper or official engineering report when available.
2. Record parameter/artifact shape, context and output constraints, required
   kernels/runtime features, quantization provenance, and expected hardware fit
   as discovery evidence.
3. Distinguish specialized next-edit, FIM/completion, general semantic, and
   teacher/critic roles. Do not compare general coding scores as next-edit
   quality.
4. Map the artifact to a complete benchmark system bundle and identify the
   smallest local compatibility probe.
5. Recommend exactly one action: `benchmark`, `compatibility spike`, `watch`, or
   `reject`, with conflicts and the evidence needed to advance.

## Evidence rules

Vendor, paper, model-card, and community claims are E0 discovery. An immutable
artifact and compatible API are not local qualification. A healthy endpoint is
not a deployed Edit policy, and decode throughput is not editor TTRS.

Pin model and tokenizer revisions, conversion/quantization digest, native
protocol revision, runtime image/flags, hardware class, context/output matrix,
and license/redistribution limits before a reproducible E2 result. Report
protocol-native comparisons as system bundles.

This skill is read-only unless the user separately authorizes download,
conversion, serving, route mutation, benchmarking on live hardware, or
promotion. Keep private host assignments and active aliases out of public
artifacts.

## Keep this skill current

After an authorized change alters model roles, admission evidence, serving
boundaries, or benchmark requirements, use `$refresh-product-guidance` before
handoff. Update this skill in the same change when its trigger or workflow is
stale. On read-only intake, report drift without editing repository files.
