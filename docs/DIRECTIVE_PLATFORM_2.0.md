<!-- Source of truth: this file is the canonical copy of the "0.4" / PLATFORM 2.0
     directive. Do not summarise, truncate, or reorder its content. -->

# ⛔ ОБЯЗАТЕЛЬНО К ВЫПОЛНЕНИЮ — DIRECTIVE: PLATFORM 2.0

> **Статус: MANDATORY / обязателен к исполнению в полном объёме.**
>
> Этот документ — не идея и не пожелание, а **директива**. Она обязательна к
> выполнению целиком.

## Правила исполнения (не подлежат обсуждению)

1. **Прочитать все строки без единого исключения.** Не пропускать, не
   «пробегать по диагонали», не резюмировать вместо чтения. Каждая строка ниже
   имеет значение.
2. **Выполнить всё, что здесь сказано, на высшем уровне** — так, как это делают
   лучшие инженерные команды мира: продуманно, профессионально, до конца.
3. **Максимальная безопасность превыше скорости.** Никаких секретов в репозитории,
   принцип наименьших привилегий, явные security-состояния, изоляция денежного и
   withdrawal-контура. При конфликте «быстро» и «безопасно» — выбирается
   безопасно.
4. **Не ломать то, что уже работает.** Сначала изучить текущий репозиторий и
   определить, что уже реализовано; расширять, а не переписывать рабочее ядро
   (см. `DO NOT restart / DO NOT rewrite` ниже).
5. **Честность вместо видимости.** Никаких выдуманных цифр, фиктивных статусов и
   «потёмкинских» фич. Заявлено — значит реально работает и проверено.
6. **Каждый заявленный пункт закрывается доказательством** — коммит,
   опубликованный артефакт или живой URL.

## Как применять

- Считать этот файл **источником истины** для развития продукта до PLATFORM 2.0.
- Выполнять поэтапно, сверяясь с уже готовой синхронизацией `metadata/*.json` →
  сайт и с принципами приватности/офлайна.
- Любое отклонение от директивы фиксировать явно и обосновывать, а не замалчивать.

---

<!-- Ниже — полный, неизменённый текст директивы (файл «0.4»). Читать целиком. -->


Осталось намеренно (пункты 1 и 4)

- №1 — реальные цифры на железе: нужно поставить модель и прогнать бенчмарки на настоящем ПК (без выдуманных чисел).
- №4 — macOS Intel (x64): вернуть, когда будет стабильный раннер.


Да. Я бы здесь уже поднял проект на следующий уровень: **TurkmenAI Local должен стать не только локальным launcher'ом моделей, а AI platform + model/dataset/training marketplace + universal API gateway**.

И я бы добавил ещё одну очень важную вещь: **один интерфейс для Local + BYOK + Third-party APIs + Marketplace providers**. То есть пользователь может запустить локальную модель, вставить свой OpenAI/Anthropic/OpenRouter/API другого провайдера или выбрать API продавца прямо из TurkmenAI. Для unified gateway логика вроде LiteLLM полезна как upstream/reference: он уже унифицирует десятки LLM-провайдеров через OpenAI-compatible interface, маршрутизацию, fallback, virtual keys, бюджеты и spend tracking. ([LiteLLM][1]) OpenRouter показывает ещё более близкую к твоей идею marketplace: единый OpenAI-compatible API, маршрутизация по цене/latency/uptime и провайдерская модель с оплатой поставщикам. ([OpenRouter][2])

Для платежей я бы не писал свою финансовую систему: **Stripe Connect** уже предназначен для marketplace-сценария с продавцами, onboarding/KYC, application fee, payouts, refunds и disputes. ([Stripe Docs][3]) При этом доступность Stripe/Connect и payout methods зависит от страны и юридической структуры, поэтому это должно быть абстракцией `PaymentProvider`, а не жёстко зашитым assumption.

И ещё: API нельзя проектировать только вокруг старого `/chat/completions`. Современный OpenAI-подобный gateway должен иметь Responses-style endpoint, streaming events, tools, structured outputs, model listing и т.д.; OpenAI сейчас документирует Responses API как основной современный интерфейс с streaming, tools и JSON Schema structured outputs. ([OpenAI Platform][4])

Ниже — **большой финальный prompt для Claude Code**, который я бы добавил к существующему репозиторию. Он уже учитывает твою новую идею: **модели + датасеты + обучение + внешние API + продавцы + marketplace + оплата + unified console + OpenAI compatibility + Туркменистан/плохой интернет + SEO + production UX.**

# TURKMENAI LOCAL

# PLATFORM 2.0

## Universal Local AI + Model Marketplace + Dataset Marketplace + Training Advisor + API Gateway

This document extends the existing TurkmenAI Local architecture.

DO NOT restart the project.

DO NOT rewrite the current working core.

FIRST inspect the current repository and determine which parts already exist.

The goal is to evolve TurkmenAI Local from a local AI desktop control plane into a complete AI platform.

The final product should combine:

1. Local AI runtime
2. Model marketplace
3. Dataset marketplace
4. Training advisor
5. Dataset downloader
6. Model downloader
7. Hardware advisor
8. Automatic quantization/optimization
9. Local OpenAI-compatible API
10. External API gateway
11. BYOK API support
12. API marketplace
13. Third-party AI provider marketplace
14. Provider onboarding
15. Usage metering
16. Billing and credits
17. Seller payouts
18. Model/API publishing
19. Unified console
20. Offline-first operation
21. Poor-internet optimization
22. Professional public website
23. Search-engine optimization
24. Public documentation
25. Cross-platform releases

The guiding principle:

> ONE APPLICATION.
> ONE CONSOLE.
> MANY LOCAL MODELS.
> MANY DATASETS.
> MANY TRAINING OPTIONS.
> MANY API PROVIDERS.
> ONE UNIFIED API.

============================================================

1. CURRENT PROJECT FIRST
   ============================================================

Inspect the existing TurkmenAI Local repository.

Do not assume anything.

Determine current:

* architecture
* desktop
* Rust core
* frontend
* backend
* model registry
* hardware detection
* downloader
* API
* runtime
* website
* release pipeline

Reuse all good existing code.

Create/update:

docs/PLATFORM_2_ARCHITECTURE.md

============================================================
2. PRODUCT TRANSFORMATION
=========================

The product must evolve from:

"local model control plane"

into:

# TurkmenAI Platform

with two equally important sides:

## LOCAL

Run AI locally.

## CLOUD / API

Use remote AI providers through APIs.

The user should not care whether the model is:

* local;
* self-hosted;
* LAN;
* third-party API;
* OpenAI;
* Anthropic;
* OpenRouter;
* another provider;
* a TurkmenAI marketplace provider.

The console presents one unified model/provider experience.

============================================================
3. UNIVERSAL MODEL ABSTRACTION
==============================

Create a provider-independent model identity.

A model entry can represent:

LOCAL_MODEL
REMOTE_API_MODEL
MARKETPLACE_MODEL
SELF_HOSTED_MODEL
LAN_MODEL

Each model has:

* provider
* model ID
* capabilities
* modality
* context
* pricing if remote
* hardware requirements if local
* latency/performance if known
* availability
* status
* source

============================================================
4. UNIVERSAL PROVIDER ABSTRACTION
=================================

Create:

Provider

with capabilities:

```text
chat
responses
embeddings
images
audio
reranking
moderation
tools
structured_output
streaming
batch
```

Providers may be:

* local runtime
* OpenAI
* Anthropic
* OpenRouter
* custom OpenAI-compatible API
* TurkmenAI marketplace provider
* self-hosted server
* LAN server

Do NOT hardcode providers into UI logic.

============================================================
5. UNIFIED MODEL CATALOG
========================

The Models page must combine:

Local models
Hugging Face models
TurkmenAI models
Remote API models
Marketplace models

with clear badges:

LOCAL
API
MARKETPLACE
LAN
VERIFIED
EXPERIMENTAL

============================================================
6. FILTER SYSTEM
================

Every model/catalog view supports filters.

For models:

* compatible with my PC
* installed
* downloadable
* local
* API
* free
* paid
* Turkmen
* Russian
* English
* coding
* chat
* vision
* audio
* embeddings
* reasoning
* small
* fast
* highest quality

For datasets:

* compatible with my PC
* language
* task
* size
* modality
* license
* format
* training type
* verified
* local
* Hugging Face
* free
* paid

For API providers:

* free
* paid
* price
* latency
* location
* reliability
* capabilities
* verified

CRITICAL:

When the user asks to download a model, download a dataset, or train a model, the UI MUST prioritize options that are actually compatible with the user's hardware.

============================================================
7. "ONLY SHOW WHAT MY PC CAN HANDLE"
====================================

Add:

"Compatible with my PC"

filter.

This must be ON by default in recommendation flows.

For models:

show only models for which there is a valid execution plan.

For datasets:

show only datasets that can realistically be used for the selected training task.

For training:

show only combinations that can actually run.

Allow:

"Show incompatible"

as an optional override.

============================================================
8. MODEL CARD
=============

Every model card should show only essential information first:

Model name
Short description
Task
Language
Size
Hardware fit
Recommended variant
Estimated speed
License
Source

Example:

```text
Turkmen ASR Large

Automatic speech recognition
Turkmen

Size:
1.8 GB

Your PC:
✓ Compatible

Recommended:
CTranslate2 INT8

[Download]
```

Advanced information remains expandable.

============================================================
9. DATASET SYSTEM
=================

Datasets become first-class resources.

Create:

DatasetRegistry
DatasetResolver
DatasetDownloader
DatasetInspector

Support:

* Hugging Face datasets
* local datasets
* direct dataset archives
* Parquet
* JSON
* JSONL
* CSV
* TSV
* TXT
* WebDataset
* Arrow
* audio datasets
* image datasets
* multimodal datasets

Do not assume every dataset can be trained directly.

============================================================
10. DATASET INSPECTOR
=====================

Before download or training:

analyze dataset metadata.

Show:

* size
* number of rows
* columns
* modalities
* language
* sample count
* audio duration
* image count
* text size
* labels
* format
* license
* estimated disk space
* estimated training storage
* preprocessing requirements

If metadata is available without downloading the full dataset, use it.

============================================================
11. DATASET DOWNLOAD ENGINE
===========================

Use the same resilient download architecture as model downloads.

Required:

* resume
* retries
* chunking
* hash verification
* queue
* scheduling
* bandwidth limiting
* partial recovery
* deduplication
* selective files
* shard-aware downloads

This is critical for Turkmenistan.

============================================================
12. DATASET PREVIEW
===================

Before downloading:

show:

```text
Dataset:
...

Size:
8.7 GB

Samples:
...

Your disk:
...

Estimated download:
...

Your network:
...

Expected time:
...
```

Then:

[Download]

============================================================
13. DATASET HARDWARE ADVISOR
============================

A dataset itself does not "fit" a GPU in the same way a model does.

Therefore calculate:

* dataset size
* preprocessing memory
* temporary storage
* RAM required
* VRAM required for training
* expected cached dataset size
* expected output checkpoint size

The result should be:

SAFE
POSSIBLE
HEAVY
NOT RECOMMENDED

============================================================
14. TRAINING ADVISOR
====================

Create:

# Training Advisor

User chooses:

Task:

* fine-tuning
* LoRA
* QLoRA
* full fine-tuning
* continued pretraining
* embedding fine-tuning
* classifier training
* ASR fine-tuning
* TTS training

Then the advisor analyzes:

* GPU
* VRAM
* RAM
* CPU
* storage
* dataset
* model
* training method

and returns:

CAN RUN
CAN RUN WITH LIMITS
NOT RECOMMENDED
CANNOT RUN

============================================================
15. TRAINING RECOMMENDATION
===========================

Example:

```text
Your computer:

RTX 3050
8 GB VRAM
32 GB RAM

Selected:
7B LLM
Dataset:
40k samples

Recommendation:

QLoRA
4-bit base model

Context:
2048

Batch:
1

Gradient accumulation:
8

Expected training:
Possible

Full fine-tuning:
Not recommended
```

Never claim exact training time unless measured or conservatively estimated.

============================================================
16. TRAINING STRATEGY LADDER
============================

When full training doesn't fit:

automatically suggest:

1. LoRA
2. QLoRA
3. gradient accumulation
4. smaller batch
5. lower sequence length
6. gradient checkpointing
7. 4-bit loading
8. CPU offload where supported
9. smaller model
10. subset dataset

The goal:

> achieve the user's training objective with the smallest feasible hardware.

============================================================
17. TRAINING HARDWARE PROFILE
=============================

Create training-specific hardware calculations.

Track:

* VRAM
* RAM
* GPU
* GPU count
* storage speed
* storage capacity
* CPU
* mixed precision support

============================================================
18. TRAINING PLAN OBJECT
========================

Create:

TrainingPlan

Example:

```json
{
  "base_model": "...",
  "dataset": "...",
  "method": "qlora",
  "precision": "4bit",
  "batch_size": 1,
  "gradient_accumulation": 8,
  "sequence_length": 2048,
  "checkpointing": true,
  "output_dir": "...",
  "estimated_storage_gb": 12
}
```

The UI generates this automatically.

============================================================
19. TRAINING BACKENDS
=====================

Do not create training frameworks from scratch.

Integrate mature ecosystems where suitable:

* Transformers Trainer
* TRL
* PEFT
* bitsandbytes
* TorchAO
* Accelerate
* Unsloth where compatible
* Axolotl where appropriate
* existing ASR training frameworks
* model-specific official training code

Select backend based on model/task.

============================================================
20. LORA / QLORA
================

Make:

LoRA
QLoRA

first-class training methods.

These should be the default recommendations for consumer GPUs when suitable.

============================================================
21. TRAINING JOB MANAGER
========================

Training jobs must support:

* create
* queue
* pause where safe
* cancel
* resume where supported
* logs
* metrics
* checkpoint list
* output path
* final model/adapter registration

============================================================
22. TRAINING DASHBOARD
======================

Show:

* loss
* learning rate
* step
* epoch
* ETA
* GPU usage
* VRAM
* RAM
* checkpoint
* output files

Only show metrics actually available.

============================================================
23. CHECKPOINT MANAGEMENT
=========================

Support:

* save
* resume
* delete
* export
* register result

Do not lose completed checkpoints because a job failed.

============================================================
24. TRAINING RECOVERY
=====================

If the application closes:

recover job state.

If backend supports resume:

offer resume.

============================================================
25. TRAINING OUTPUT AS A MODEL
==============================

After training:

the output can be registered as:

MODEL
ADAPTER
CHECKPOINT

The model library should recognize it.

============================================================
26. TRAINING PIPELINE
=====================

Pipeline:

```text
Model
↓
Dataset
↓
Dataset inspection
↓
Hardware analysis
↓
Training method
↓
Training plan
↓
Download dependencies
↓
Train
↓
Validate
↓
Register output
```

============================================================
27. TRAINING VALIDATION
=======================

After training:

perform basic:

* file validation
* load test
* inference test
* adapter/base compatibility check

Mark:

TRAINED
VALIDATED
FAILED

============================================================
28. DATASET QUALITY ANALYSIS
============================

Before training, offer:

Dataset Quality Report

Analyze:

* duplicates
* empty rows
* malformed examples
* missing values
* outliers
* distribution
* class imbalance
* language distribution
* audio duration distribution
* invalid paths

Never execute arbitrary dataset code.

============================================================
29. DATASET CLEANUP
===================

Where practical allow safe operations:

* dedup
* remove empty rows
* normalize fields
* split train/validation/test
* detect corrupted files

Never modify original dataset destructively.

Create derived dataset.

============================================================
30. DATASET DERIVATION
======================

Use:

Original dataset
+
transform configuration
=======================

Derived dataset

Store derivation metadata.

============================================================
31. DATASET VERSIONING
======================

Track:

* source
* revision
* hash
* transformations
* output hash

============================================================
32. DATASET LICENSE
===================

Every dataset card must display:

* license
* source
* redistribution status

============================================================
33. DATASET MARKETPLACE
=======================

Create marketplace category:

# Datasets

Users can:

* discover
* inspect
* download
* use in training
* publish
* sell where legally permitted

Do not imply ownership of third-party datasets.

============================================================
34. MODEL MARKETPLACE
=====================

Create marketplace:

# Models

Resources may be:

* Free
* Paid
* Open source
* API
* Local
* Downloadable
* Fine-tuned
* Adapters

============================================================
35. API MARKETPLACE
===================

This is a major feature.

Create:

# API Marketplace

Anyone who can legally provide an API can register a provider.

A provider can publish:

* model name
* model ID
* endpoint
* capabilities
* context
* pricing
* region
* limits
* uptime
* privacy policy
* terms
* documentation

The provider must implement the TurkmenAI provider contract.

============================================================
36. OPENAI COMPATIBILITY
========================

The marketplace should strongly prefer providers that implement an OpenAI-compatible API.

Minimum:

POST /v1/chat/completions
GET /v1/models

Preferred modern contract:

POST /v1/responses

plus:

* streaming
* structured output
* tool calling
* embeddings
* audio where applicable

The platform should expose one normalized API contract to application developers.

============================================================
37. EXTERNAL API KEY SUPPORT
============================

Inside TurkmenAI Console users can add:

* OpenAI
* Anthropic
* OpenRouter
* custom OpenAI-compatible provider
* any other supported provider

Use secure credential storage.

Never send credentials to the wrong provider.

============================================================
38. "ADD YOUR API"
==================

Create a developer-friendly setup:

```text
Provider name:
...

Base URL:
https://example.com/v1

API key:
••••••••

Model:
...

Test connection
```

TurkmenAI automatically calls:

* models endpoint
* capability test
* minimal inference test

Then shows:

CONNECTED
or
FAILED

============================================================
39. API PROVIDER VALIDATION
===========================

For a custom provider test:

1. validate URL
2. authenticate
3. list models
4. validate response format
5. test streaming
6. test usage data
7. detect supported capabilities

Then generate a ProviderProfile.

============================================================
40. PROVIDER CAPABILITY MATRIX
==============================

Each provider/model declares:

* chat
* responses
* vision
* audio
* embeddings
* tools
* structured output
* streaming
* batch
* reasoning
* image generation

Do not expose unsupported features.

============================================================
41. API ROUTER
==============

Create:

UniversalRouter

Input:

application request

Output:

best provider/model.

Potential routing dimensions:

* local vs cloud
* price
* latency
* uptime
* context
* capabilities
* region
* privacy
* user preference

============================================================
42. LOCAL-FIRST ROUTING
=======================

Default behavior:

If a compatible local model exists and the user selected local-first:

prefer local inference.

Only route to remote if:

* user chose remote
* local unavailable
* fallback explicitly enabled

Never silently send local requests to cloud.

============================================================
43. SMART FAILOVER
==================

For API mode:

Provider A fails
↓
Provider B
↓
Provider C

Only if:

* compatible
* user permits fallback
* privacy policy is compatible

Never fail over to a provider that violates the selected privacy policy.

============================================================
44. PRIVACY ROUTING
===================

User can select:

LOCAL ONLY
EU ONLY
SPECIFIC PROVIDER
NO TRAINING
NO LOGGING
ANY AVAILABLE

The router must respect policy.

============================================================
45. API PRICING
===============

Provider metadata can specify:

* input price
* output price
* cached input price
* image price
* audio price
* request fee
* minimum cost

Use normalized currency units internally.

Never hardcode provider pricing.

============================================================
46. USAGE METERING
==================

For API traffic track:

* request count
* input tokens
* output tokens
* cached tokens where available
* latency
* status
* estimated cost
* provider
* model

Never log raw prompts by default.

============================================================
47. USAGE DASHBOARD
===================

Create:

# Usage

Show:

Today
This week
This month

Metrics:

* requests
* tokens
* cost
* average latency
* errors
* model usage

============================================================
48. API KEYS FOR LOCAL USERS
============================

Let developers create TurkmenAI API keys.

Example:

```text
tkai_live_...
```

Keys must support:

* create
* revoke
* rotate
* scope
* last used
* created date

Store hashed keys where possible.

Never display full secret again after creation.

============================================================
49. PROJECTS
============

Inspired by modern AI platforms:

users can create projects:

Project:
Telegram Bot

API Keys:
key_1

Models:
best-turkmen

Budget:
$10

Usage:
...

============================================================
50. API KEY SCOPES
==================

Scopes:

* inference
* embeddings
* audio
* images
* models:read
* usage:read
* admin

Principle of least privilege.

============================================================
51. VIRTUAL KEYS / SERVICE KEYS
===============================

Allow project-specific API keys.

Useful for:

* apps
* bots
* teams
* development
* production

============================================================
52. RATE LIMITS
===============

Support:

* requests/min
* tokens/min
* daily quota
* monthly budget

============================================================
53. BUDGETS
===========

User can set:

Daily:
$1

Monthly:
$20

When exceeded:

stop
or
fallback local

depending on configuration.

============================================================
54. API CONSOLE
===============

Create a professional developer console:

Overview
Projects
API Keys
Models
Providers
Usage
Billing
Logs
Webhooks
Settings

============================================================
55. API LOGS
============

Show sanitized metadata:

* timestamp
* request ID
* model
* provider
* latency
* tokens
* status
* estimated cost

Prompt/completion content OFF by default.

============================================================
56. REQUEST ID
==============

Every API request gets:

request_id

This must appear in:

* API response headers where appropriate
* logs
* dashboard

============================================================
57. IDEMPOTENCY
===============

Support idempotency for mutation/payment endpoints where appropriate.

============================================================
58. WEBHOOKS
============

Allow providers/apps to receive:

* usage events
* job events
* model events
* billing events

Secure with:

* signing secret
* retry
* event IDs
* replay protection

============================================================
59. BATCH API
=============

Where backend supports batch workloads:

allow asynchronous jobs.

Useful for:

* dataset preprocessing
* embeddings
* evaluation
* large offline API jobs

Do not implement an imitation if a suitable backend already provides it.

============================================================
60. STREAMING
=============

Use streaming end-to-end.

Support normalized events.

Do not collapse everything into one final response.

Modern Responses-style APIs use structured streaming events; preserve that event-oriented architecture rather than only emulating old Chat Completions. ([OpenAI Platform][4])

============================================================
61. RESPONSES API COMPATIBILITY
===============================

Implement a modern normalized response endpoint:

POST /v1/responses

Support where provider permits:

* input
* instructions
* previous response IDs
* tools
* structured outputs
* streaming
* reasoning metadata
* multimodal content

Map local backends to this normalized representation.

============================================================
62. BACKWARD COMPATIBILITY
==========================

Also maintain:

POST /v1/chat/completions

for ecosystem compatibility.

============================================================
63. EMBEDDINGS API
==================

Expose:

POST /v1/embeddings

for compatible local and remote models.

============================================================
64. AUDIO API
=============

Where implemented:

POST /v1/audio/transcriptions
POST /v1/audio/speech

============================================================
65. IMAGE API
=============

Where implemented:

POST /v1/images/generations

============================================================
66. MODEL LIST API
==================

GET /v1/models

must return normalized model metadata.

============================================================
67. OPENAI-COMPATIBLE CLI
=========================

Provide developer copy-paste examples.

Python
JavaScript
curl

============================================================
68. PROVIDER PUBLISHING
=======================

Create:

Developer -> Become a Provider

Steps:

1. create provider
2. verify endpoint
3. publish models
4. define capabilities
5. define pricing
6. define privacy
7. test endpoint
8. submit/publish

============================================================
69. PROVIDER VERIFICATION
=========================

Statuses:

Unverified
Tested
Verified
Official

Verification should require actual automated checks.

Never automatically grant Verified.

============================================================
70. PERFORMANCE METRICS
=======================

Track provider:

* TTFT
* tokens/sec
* uptime
* error rate
* timeout rate

Use rolling windows.

Do not manipulate rankings dishonestly.

OpenRouter's provider model is a useful reference: providers expose OpenAI-compatible endpoints and are evaluated on latency, throughput, uptime and price. ([OpenRouter][2])

============================================================
71. PROVIDER ROUTING
====================

Ranking can consider:

* price
* latency
* throughput
* uptime
* capability match
* geography
* user privacy policy

Provide a transparent "Why this provider?" explanation.

============================================================
72. API MARKETPLACE CARD
========================

Show:

Provider
Model
Price
Context
Latency
Capabilities
Region
Uptime
Verification
Privacy
[Try]
[Use API]
[Add Key]

============================================================
73. SELLER MODEL
================

Users should be able to sell legally owned:

* API access
* models
* adapters
* datasets
* hosted inference

where licensing permits.

============================================================
74. PAYMENT ARCHITECTURE
========================

Do NOT build a custom payment processor.

Create:

PaymentProvider abstraction.

Primary candidate:

Stripe Connect where supported and legally appropriate.

Stripe Connect is designed for marketplaces with connected accounts, seller onboarding, application fees, refunds/disputes and payouts. ([Stripe Docs][5])

But do not hardcode Stripe as the only possible provider.

Architecture must support future payment providers.

============================================================
75. SELLER ONBOARDING
=====================

Seller flow:

Become a provider
↓
Identity/business onboarding
↓
Payment account verification
↓
Provider setup
↓
Model/API listing
↓
Technical validation
↓
Publish

Use provider-hosted or embedded onboarding rather than writing custom KYC logic. Stripe explicitly recommends hosted or embedded onboarding to reduce ongoing compliance maintenance. ([Stripe Docs][6])

============================================================
76. AGE / LEGAL / COMPLIANCE
============================

Do not assume every user can legally become a seller.

Payment provider and platform rules determine seller eligibility.

Seller onboarding must handle:

* jurisdiction
* age/legal eligibility
* identity verification
* tax/business requirements
* payout availability

Never attempt to bypass payment-provider KYC.

============================================================
77. COMMISSION
==============

Platform can support:

listing fee
transaction fee
API marketplace commission

Do not hardcode final percentages.

Make commission configurable per product class.

============================================================
78. DIGITAL PRODUCT PURCHASE
============================

For paid:

* models
* datasets
* adapters
* API credits
* subscriptions

provide:

checkout
receipt
order
refund status
entitlement

============================================================
79. API CREDIT SYSTEM
=====================

Create internal wallet/credit abstraction.

Balance:
$10

usage:

* $0.13

remaining:
$9.87

Do not use floating-point arithmetic for monetary values.

Use integer minor units or decimal-safe representation.

============================================================
80. LEDGER
==========

Create immutable financial ledger.

Events:

credit_purchase
usage_charge
refund
commission
seller_credit
payout
adjustment

Never recalculate historical balances from mutable rows.

============================================================
81. PAYMENT WEBHOOKS
====================

All payment/provider status changes must be webhook-driven where the payment
provider supports it.

Handle:

* payment succeeded
* payment failed
* refund
* dispute
* payout
* connected account updates

Use idempotency/event IDs.

============================================================
82. REFUNDS
===========

Create refund state machine.

Do not implement arbitrary financial mutations from the frontend.

============================================================
83. SELLER BALANCE
==================

Seller dashboard:

Gross
Platform fee
Refunds
Net
Pending
Available
Paid out

============================================================
84. PAYOUTS
===========

Show:

* pending
* available
* scheduled
* completed
* failed

Do not promise payout timing independent of the payment provider.

============================================================
85. MARKETPLACE MODERATION
==========================

Users must not be able to publish anything without checks.

Create statuses:

Draft
Pending Review
Approved
Rejected
Suspended
Archived

============================================================
86. MODEL/ DATASET LICENSE REVIEW
=================================

For every published item:

* source
* license
* publisher
* usage rights
* redistribution rights

A marketplace seller must attest they have the right to sell or provide access.

============================================================
87. ABUSE REPORTING
===================

Allow:

Report model
Report dataset
Report provider
Report API

============================================================
88. TRUST AND SAFETY
====================

Create:

* provider verification
* abuse reporting
* takedown status
* suspension
* audit logs

============================================================
89. SELLER DASHBOARD
====================

Provider sees:

Overview
Models
Datasets
API endpoints
Pricing
Requests
Users
Revenue
Payouts
Quality
Logs
Settings

============================================================
90. API PROVIDER HEALTH
=======================

Provider sees:

* uptime
* request latency
* throughput
* errors
* usage
* cost
* active model status

============================================================
91. MODEL PUBLISHING
====================

Seller can publish:

local/downloadable model
or
API-backed model

If downloadable:

source files
format
license
checksums

If API:

endpoint
capabilities
pricing
privacy policy
limits

============================================================
92. MODEL HOSTING
=================

Do NOT become the default host for all third-party model weights.

Prefer:

* Hugging Face
* seller infrastructure
* verified sources

TurkmenAI marketplace stores metadata and entitlement unless actual hosting is
explicitly configured.

============================================================
93. DATASET SELLING
===================

Support dataset listings without necessarily hosting the data.

Possible delivery:

* Hugging Face gated repository
* seller URL
* downloadable package
* TurkmenAI package

============================================================
94. DATASET ACCESS ENTITLEMENTS
===============================

Paid dataset:

Purchase
↓
Entitlement
↓
Access granted

Do not expose seller files without entitlement.

============================================================
95. API SUBSCRIPTIONS
=====================

Allow:

Pay-as-you-go
Monthly credits
Project budgets

Subscriptions should remain optional.

============================================================
96. CONSOLE BILLING
===================

User console:

Billing
Credits
Payment methods
Transactions
Usage
Invoices
Refunds

============================================================
97. PROVIDER BILLING
====================

Provider:

Revenue
Fees
Payouts
Invoices
Tax/verification status

============================================================
98. PAYMENT SECURITY
====================

Never store raw card data.

Use payment provider hosted/tokenized flows.

============================================================
99. SECRET STORAGE
==================

Store:

* provider API keys
* payment credentials
* HF tokens

using secure secret mechanisms.

Never commit them.

============================================================
100. API KEY SECURITY
=====================

Keys are secrets.

Store hashed or encrypted according to the security model.

Display full secret only once.

============================================================
101. API KEY ROTATION
=====================

Support:

Create
Revoke
Rotate

============================================================
102. API USAGE LIMITS
=====================

Allow per-key:

* requests/min
* tokens/min
* daily
* monthly
* cost

============================================================
103. ABUSE PREVENTION
=====================

Implement:

* rate limiting
* IP throttling where appropriate
* key throttling
* provider throttling
* request size limits
* max output limits
* timeout limits

============================================================
104. BILLING SAFETY
===================

Never let frontend decide the final charge.

Server calculates usage charges.

============================================================
105. USAGE PRICE CALCULATION
============================

Prices must come from server-side provider/model configuration.

Never trust client-supplied price.

============================================================
106. PROVIDER RECONCILIATION
============================

Track provider usage and compare with provider-returned usage where available.

If mismatch occurs:

mark for reconciliation.

============================================================
107. PAYMENT IDENTITY
=====================

Keep:

User
Provider
Connected Account
Project
API Key
Order
Transaction
Ledger Entry

as distinct entities.

============================================================
108. ORGANIZATIONS / TEAMS
==========================

Add organization support:

Organization
Members
Roles
Projects
Billing
Keys

Roles:

Owner
Admin
Developer
Viewer

============================================================
109. PROJECTS
=============

Project has:

* API keys
* budgets
* models
* providers
* usage
* webhooks

============================================================
110. RBAC
=========

Implement role-based access.

Do not allow frontend-only permission checks.

============================================================
111. AUDIT LOG
==============

Record sensitive admin events:

* key creation
* key revocation
* provider changes
* billing actions
* model publication
* moderation
* payout status

============================================================
112. ADMIN CONSOLE
==================

Create separate admin capabilities:

* users
* providers
* models
* datasets
* payments
* disputes
* moderation
* system health
* logs

Do not expose admin tools to ordinary users.

============================================================
113. SEARCH ENGINE OPTIMIZATION
===============================

The website must be optimized to rank strongly for relevant searches.

Do not promise "first place".

Instead maximize legitimate ranking potential.

Target pages/topics:

* Turkmen AI
* Turkmen AI models
* Turkmen speech recognition
* Turkmen TTS
* local AI
* offline AI
* local LLM
* AI for Turkmenistan
* Hugging Face Turkmen models
* local AI for weak PC
* AI models offline
* local model runner
* Turkmen ASR
* Turkmen datasets
* Turkmen AI training

============================================================
114. SEO ARCHITECTURE
=====================

Create dedicated indexable pages.

Do not put everything on one SPA route.

Use static/server-rendered pages where appropriate.

Generate:

* sitemap.xml
* robots.txt
* canonical tags
* OpenGraph
* Twitter/X cards
* JSON-LD structured data

============================================================
115. MODEL SEO
==============

Public model pages should have:

* unique title
* description
* structured metadata
* source
* task
* language
* compatibility summary
* publisher

============================================================
116. DATASET SEO
================

Likewise:

* dataset name
* language
* size
* task
* source
* license
* use cases

============================================================
117. PROVIDER SEO
=================

Provider pages:

* provider
* models
* capabilities
* pricing
* regions
* verification

============================================================
118. TRAINING SEO
=================

Create educational pages:

* How to train AI locally
* How to fine-tune on 8GB VRAM
* QLoRA guide
* Turkmen ASR training
* Turkmen TTS training
* local AI with weak PC

Only factual content.

============================================================
119. PROGRAMMATIC SEO SAFETY
============================

Do not generate thousands of useless pages.

Every indexed page must provide meaningful information.

============================================================
120. WEBSITE PERFORMANCE
========================

Target:

* fast first load
* optimized images
* small JS
* lazy loading
* accessible HTML
* responsive design

============================================================
121. WEBSITE DESIGN
===================

Make it look like a top-tier AI product.

Visual language:

* clean
* technical
* premium
* restrained
* dark/light themes
* strong typography
* real product screenshots
* real data

Avoid generic AI gradients and template-looking UI.

============================================================
122. DESKTOP CONSOLE DESIGN
===========================

Main navigation:

Home
Models
Datasets
Training
API
Providers
Marketplace
Downloads
Chat
Usage
Storage
Hardware
Settings

Hide marketplace/payment items if the deployment does not have the required
backend configured, but keep architecture ready.

============================================================
123. HOME DASHBOARD
===================

Show:

Computer
Recommended model
Recommended dataset
Training recommendation
Current downloads
API status
Installed models
Storage
Usage

============================================================
124. UNIVERSAL SEARCH
=====================

Search everything:

Models
Datasets
Providers
Apps
Documentation

============================================================
125. COMMAND PALETTE
====================

Ctrl/Cmd + K.

Commands:

Search model
Search dataset
Run model
Download
Start training
Open API
Add provider
Add API key
Check hardware
Open docs

============================================================
126. "WHAT CAN I DO WITH THIS PC?"
==================================

Create a major screen.

Input:
Nothing required.

Output:

Models I can run
Datasets I can train on
Training methods I can use
Voice models I can run
Vision models I can run
Recommended AI stack

This becomes a defining TurkmenAI feature.

============================================================
127. RECOMMENDED STACK
======================

Generate:

My AI Stack

Example:

LLM:
8B Q4

ASR:
Turkmen ASR small

TTS:
Turkmen TTS

Embeddings:
multilingual

Training:
QLoRA on 3B/7B

Total disk:
...

============================================================
128. MODEL/DATASET/TRAINING FILTER
==================================

When the user enters:

"Download model"

show only:

models compatible with current hardware by default.

When:

"Download dataset"

show datasets that fit selected training plans.

When:

"Train"

show only valid:

model + dataset + method

combinations.

Each item has a short description.

============================================================
129. TRAINING COMBINATION VIEW
==============================

Example:

```text
Model:
Qwen 7B

Dataset:
40k Turkmen instruction examples

Method:
QLoRA

Your hardware:
RTX 3050 8GB

Status:
✓ Possible

Estimated storage:
...
```

============================================================
130. ONE-CLICK TRAINING
=======================

Button:

Start Training

The application creates:

* environment
* dataset derivation
* training configuration
* output directory

where appropriate.

Do not require terminal.

============================================================
131. TRAINING SAFETY
====================

Before training:

show:

* expected disk
* expected RAM
* expected VRAM
* expected time estimate if only approximate
* selected method
* output directory

Require confirmation for large jobs.

============================================================
132. TRAINING PRESETS
=====================

Simple:

Fast
Balanced
Quality
Low VRAM

Advanced:

all training hyperparameters.

============================================================
133. TRAINING ARTIFACT REGISTRATION
===================================

After training:

offer:

Register model
Register adapter
Export
Publish

============================================================
134. MODEL PUBLISH FLOW
=======================

Developer:

Create listing
↓
Upload metadata
↓
Connect source
↓
License
↓
Capabilities
↓
Pricing
↓
Verification
↓
Publish

============================================================
135. API PUBLISH FLOW
=====================

Developer:

Create provider
↓
Base URL
↓
API auth
↓
Test
↓
Capabilities
↓
Models
↓
Pricing
↓
Privacy
↓
Publish

============================================================
136. CUSTOM OPENAI-COMPATIBLE PROVIDER
======================================

The console must allow:

Provider:
MyCompany

Base URL:
[https://api.example.com/v1](https://api.example.com/v1)

API Key:
...

Then:

Test

and automatically discover:

models
capabilities
usage format

============================================================
137. API PLAYGROUND
===================

Create:

# API Playground

User can:

* select provider
* select model
* enter prompt
* attach image where supported
* add tools
* switch streaming
* inspect request
* inspect response
* inspect latency
* inspect token usage
* inspect estimated cost

============================================================
138. CODE EXPORT
================

Button:

Copy code

Generate:

Python
JavaScript
curl

using the chosen provider/model.

============================================================
139. API ENDPOINT DOCUMENTATION
===============================

Provide:

Base URL
Authentication
Models
Responses
Chat Completions
Embeddings
Audio
Images
Errors
Streaming
Tools
Structured output
Rate limits
Usage

============================================================
140. ERROR MODEL
================

Normalize provider errors:

401
403
404
409
413
429
500
502
503
504

Map them to consistent TurkmenAI error responses.

============================================================
141. RETRY POLICY
=================

Retry only safe transient failures.

Respect:

Retry-After

where available.

============================================================
142. TIMEOUTS
=============

Provider-specific timeouts.

Do not hang indefinitely.

============================================================
143. CIRCUIT BREAKER
====================

If provider repeatedly fails:

temporarily reduce traffic.

============================================================
144. PROVIDER HEALTH
====================

Keep rolling health data.

Do not route blindly to unhealthy endpoints.

============================================================
145. CACHING
============

Use appropriate caching for:

* model metadata
* registry
* repeated identical requests where legally/safely appropriate
* embeddings where safe

Never cache private data across users.

============================================================
146. PROMPT CACHING
===================

Where backend supports it:

surface prompt caching capability.

Do not assume all providers implement it identically.

============================================================
147. DATA RETENTION
===================

Default:

do not store prompts/completions.

Usage metadata can be stored.

Provider-specific logging policies must be visible.

============================================================
148. PRIVACY ROUTING
====================

Providers declare:

* logs prompts?
* retention?
* training?
* region?
* data processing location?

Users can filter accordingly.

============================================================
149. MARKETPLACE TRUST
======================

Badges:

Verified
Official
Community
Experimental

Verification criteria must be documented.

============================================================
150. REVIEWS
============

Future-ready:

ratings/reviews for providers/models.

Do not allow review manipulation.

============================================================
151. ABUSE / REPORTING
======================

Report:

model
dataset
provider
API
user

Admin workflow:

review
warn
suspend
remove

============================================================
152. LEGAL BOUNDARIES
=====================

Do not attempt to create a global marketplace legal framework in code.

Make policies configurable.

Payment provider / legal jurisdiction determines:

* KYC
* payouts
* taxes
* refunds
* seller eligibility

Do not bypass these systems.

============================================================
153. BILLING ARCHITECTURE
=========================

Separate:

Catalog
Order
Payment
Ledger
Entitlement
Usage
Commission
Payout

Do not put billing logic inside UI components.

============================================================
154. MONEY REPRESENTATION
=========================

Never use binary floating point for money.

Use integer minor units or a decimal type.

============================================================
155. BILLING IDEMPOTENCY
========================

All payment webhooks and financial mutations must be idempotent.

============================================================
156. RECONCILIATION
===================

Create reconciliation tools for:

orders
payments
usage
seller balances
payouts

============================================================
157. REFUNDS
============

Support:

requested
approved
processing
completed
failed

============================================================
158. PROVIDER PAYOUTS
=====================

Do not manually calculate/transfer money if payment provider can handle it.

Use provider-native marketplace payouts.

============================================================
159. STRIPE CONNECT
===================

Where Stripe is selected, use current Stripe Connect marketplace architecture.

Prefer provider-hosted or embedded onboarding rather than custom KYC.

Connected accounts are appropriate for seller/provider separation and payouts. ([Stripe Docs][5])

Do not assume Stripe is available in every jurisdiction.

PaymentProvider abstraction must remain.

============================================================
160. PAYMENT METHODS
====================

Do not promise a particular payment method globally.

Show methods actually enabled for the current deployment and jurisdiction.

============================================================
161. SELLER EXPERIENCE
======================

Provider can:

Create profile
Connect payment account
Add endpoint
Validate
Set price
Publish
See usage
See earnings
Withdraw/payout according to provider rules

============================================================
162. CUSTOMER EXPERIENCE
========================

User can:

Browse
Compare
Try
Buy credits
Use
Track usage
Download receipts
Request refunds through supported workflow

============================================================
163. CREDIT EXPIRATION
======================

Do not invent expiration policies.

Use configured terms and payment-provider constraints.

============================================================
164. FREE / PAID
================

Support:

Free models
Free APIs
Paid APIs
Paid datasets
Paid models
Trial credits

============================================================
165. TRIAL CREDITS
==================

Optional.

Must have:

* explicit amount
* expiration
* anti-abuse controls

============================================================
166. FREE PROVIDER
==================

Providers can offer free quotas.

============================================================
167. BYOK
=========

Allow user-provided provider keys.

BYOK usage should remain clearly separated from marketplace billing.

============================================================
168. PROVIDER KEY SECURITY
==========================

Encrypt or use OS secret store.

============================================================
169. CUSTOM ENDPOINT
====================

Any OpenAI-compatible API can be added manually.

This is mandatory.

============================================================
170. UNIVERSAL LOCAL + REMOTE API
=================================

The same application can expose:

Local model
or
Remote provider

through one normalized OpenAI-compatible interface.

============================================================
171. ROUTING POLICY
===================

User can create:

Local-first
Cloud-first
Cheapest
Fastest
Private-only
EU-only
Custom

============================================================
172. FALLBACK POLICY
====================

Per project:

Fallback local
Fallback provider
No fallback

============================================================
173. FAILOVER PRIVACY
=====================

Never silently send private requests to another provider.

Fallback policy must be explicit.

============================================================
174. MODEL ACCESS POLICY
========================

Projects can restrict:

* allowed providers
* allowed models
* max cost
* allowed regions
* local-only

============================================================
175. ORGANIZATIONS
==================

Add:

Organization
Members
Roles
Projects
Billing
Providers
Usage

============================================================
176. TEAM API
=============

Organization can create project API keys.

============================================================
177. RBAC
=========

Roles:

Owner
Admin
Developer
Viewer
Billing

============================================================
178. AUDIT LOG
==============

Admin actions are logged.

============================================================
179. WEBHOOKS
=============

Support event types:

usage
model.ready
training.started
training.completed
payment.succeeded
payment.failed
refund
payout
provider.health

============================================================
180. API IDEMPOTENCY
====================

Support idempotency keys for relevant requests.

============================================================
181. API VERSIONING
===================

Use:

/v1

Do not break contracts.

============================================================
182. MODERN API
===============

Implement:

/v1/responses

and:

/v1/chat/completions

Responses-style events should remain first-class in the internal abstraction.
Modern Responses API supports streaming events, tool selection, structured
outputs and conversation state concepts. ([OpenAI Platform][4])

============================================================
183. BACKWARD COMPATIBILITY
===========================

Existing OpenAI SDK clients should work with:

base_url = TurkmenAI API

as far as implemented features permit.

============================================================
184. API DOCS
=============

Generate OpenAPI specification.

Provide:

Swagger/interactive docs

only if suitable for the security model.

============================================================
185. LOCAL API
==============

Default:

127.0.0.1

For external/public API:

separate authenticated server component.

Do not expose desktop loopback API directly to the Internet.

============================================================
186. PUBLIC API ARCHITECTURE
============================

If marketplace/public APIs are implemented:

Use a server-side API gateway separate from the desktop application.

Architecture:

Internet
↓
API Gateway
↓
Authentication
↓
Rate Limit
↓
Billing
↓
Router
↓
Provider

```

The desktop app is NOT the public marketplace backend.

============================================================
187. DATABASE
============================================================

Public backend can use a server database.

Use:

- PostgreSQL or another mature transactional DB

for:

- users
- organizations
- providers
- products
- orders
- ledger
- usage
- API keys
- webhooks
- permissions

Desktop remains local-first.

============================================================
188. REDIS/QUEUE
============================================================

Use Redis or equivalent only if needed for:

- rate limits
- queue
- caching
- transient state

Do not add infrastructure without need.

============================================================
189. OBJECT STORAGE
============================================================

For public assets:

use S3-compatible object storage where necessary.

Do not host giant model files in the transactional database.

============================================================
190. ASYNC JOBS
============================================================

Background services for:

- model indexing
- dataset analysis
- training jobs
- benchmarks
- billing reconciliation
- provider health checks

============================================================
191. FRONTEND/BACKEND SEPARATION
============================================================

Desktop:

local-first.

Website:

public.

Marketplace API:

server-side.

Keep these boundaries clean.

============================================================
192. WEBSITE ACCOUNT SYSTEM
============================================================

If marketplace/API features require accounts:

provide:

Sign up
Sign in
Projects
API keys
Billing
Provider dashboard

Do NOT force accounts for local-only desktop inference.

============================================================
193. LOCAL MODE WITHOUT ACCOUNT
============================================================

Local users can:

- install app
- download local models
- run offline
- use local API

without creating an account.

============================================================
194. ACCOUNT OPTIONAL
============================================================

Account is required only for:

- marketplace purchases
- provider publishing
- cloud APIs
- synchronization
- team features

============================================================
195. SEARCH
============================================================

Create a universal search engine across:

- models
- datasets
- providers
- APIs
- docs

============================================================
196. SEARCH RANKING
============================================================

Rank by:

- relevance
- compatibility
- verified status
- quality
- performance
- freshness

Do not manipulate ranking unfairly.

============================================================
197. SEO
============================================================

The public site must be engineered for strong organic search.

Do not promise #1 position.

Create useful indexable pages.

Use:

- server/static rendering
- structured data
- canonical URLs
- internal links
- descriptive titles
- unique descriptions
- sitemap
- robots
- fast pages
- real technical content

============================================================
198. MODEL SEO PAGES
============================================================

Examples:

/models/turkmen-asr
/models/turkmen-llm

where they are actual public catalog pages.

============================================================
199. DATASET SEO PAGES
============================================================

/datasets/...

============================================================
200. PROVIDER SEO PAGES
============================================================

/providers/...

============================================================
201. TRAINING GUIDE PAGES
============================================================

/training/...

============================================================
202. NO SEO SPAM
============================================================

Do not generate thousands of thin pages.

Every page must be useful.

============================================================
203. WEBSITE INTERNAL LINKING
============================================================

Connect:

Model
-> compatible hardware
-> dataset
-> training guide
-> API
-> docs

This creates a strong knowledge graph.

============================================================
204. OPEN GRAPH
============================================================

Generate dynamic OG cards for:

- models
- datasets
- providers
- releases

============================================================
205. PERFORMANCE
============================================================

Public site target:

excellent Core Web Vitals.

Optimize:

- images
- JS
- fonts
- server rendering
- cache

============================================================
206. DESKTOP UI INFORMATION ARCHITECTURE
============================================================

Main navigation:

Home
Models
Datasets
Training
Chat
API
Marketplace
Downloads
Hardware
Storage
Usage
Settings

Developer items can be grouped.

============================================================
207. MODELS PAGE
============================================================

Top filters:

Compatible with my PC
Installed
Free
Paid
Turkmen
Task
Size
Speed

============================================================
208. DATASETS PAGE
============================================================

Top filters:

Trainable on my PC
Free
Paid
Language
Task
Size
Modality
License

============================================================
209. TRAINING PAGE
============================================================

Main flow:

Choose model
Choose dataset
Choose task
Analyze
Recommend method
Show plan
Start

============================================================
210. API PAGE
============================================================

Sections:

My API keys
Providers
Marketplace
Playground
Usage
Projects

============================================================
211. MARKETPLACE
============================================================

Tabs:

Models
Datasets
APIs
Providers
Apps

============================================================
212. API PLAYGROUND
============================================================

Must feel like a professional developer console.

Show:

provider
model
request
response
latency
tokens
cost
status
request ID

============================================================
213. CODE SNIPPETS
============================================================

Generate copy-ready:

Python
JavaScript
curl

============================================================
214. IMPORT PROVIDER
============================================================

"Add Provider"

supports:

- OpenAI-compatible URL
- API key
- model name

Automatic validation.

============================================================
215. PROVIDER TEST
============================================================

Test:

GET /models
simple completion
streaming
usage
error handling

============================================================
216. PROVIDER PUBLISHING
============================================================

Only allow publish after technical validation.

============================================================
217. PROVIDER HEALTH MONITOR
============================================================

Continuous or periodic checks:

- availability
- latency
- error rate

Do not overload providers.

============================================================
218. PROVIDER SLA
============================================================

Do not promise SLA unless business infrastructure actually provides one.

============================================================
219. API MARKETPLACE RANKING
============================================================

Recommended provider scoring:

quality
price
latency
throughput
uptime
capabilities
privacy

Keep ranking explainable.

============================================================
220. LOCAL PROVIDER
============================================================

A user's own local model can appear as:

LOCAL PROVIDER

and can be exposed to their own local apps through API.

============================================================
221. LAN PROVIDER
============================================================

Optional:

LAN provider

with explicit authentication.

============================================================
222. REMOTE SELF-HOSTED
============================================================

Users can add:

custom remote OpenAI-compatible endpoint.

============================================================
223. MODEL SOURCE TYPES
============================================================

Local
Hugging Face
Marketplace
API
LAN
Custom

============================================================
224. DATASET SOURCE TYPES
============================================================

Hugging Face
Local
Marketplace
URL
LAN
USB

============================================================
225. TRAINING SOURCE TYPES
============================================================

Local dataset
Hugging Face dataset
Marketplace dataset
Derived dataset

============================================================
226. AI STACK INSTALLER
============================================================

"Build my AI stack"

Analyzes:

- use case
- hardware
- disk
- network

Then recommends:

LLM
ASR
TTS
Embeddings
Vision
Training stack

============================================================
227. PROGRESSIVE DOWNLOAD
============================================================

This is especially important for Turkmenistan.

If a large model is downloading:

allow a smaller usable model to become READY first.

Example:

3B model ready
8B model downloading
14B model later

User can start immediately.

============================================================
228. DOWNLOAD PRIORITY
============================================================

Priority:

interactive
recommended
background

============================================================
229. MODEL UPGRADE
============================================================

User can upgrade:

3B
-> 7B
-> 14B

without losing current model.

============================================================
230. NETWORK ADAPTATION
============================================================

Measure local connection.

Adapt:

- concurrency
- chunk size
- retry
- bandwidth

Do not over-parallelize unstable connections.

============================================================
231. LAN CACHE
============================================================

If an identical model blob exists on local LAN:

allow download from LAN.

Verify hash.

============================================================
232. USB CACHE
============================================================

If a model exists on connected external storage:

offer import/reuse.

============================================================
233. OFFLINE PACKAGE
============================================================

Create portable package:

.tmai

with:

manifest
hashes
source
license
model files
runtime recommendation

============================================================
234. MODEL + DATASET BUNDLES
============================================================

Allow optional:

Training Bundle

containing:

model
dataset
training plan

This creates reproducible offline training environments.

============================================================
235. TRAINING BUNDLE IMPORT
============================================================

Another PC:

import
-> hardware analysis
-> adapt training plan
-> start

============================================================
236. HARDWARE-ADAPTIVE TRAINING
============================================================

The same training project should adapt:

RTX 4090
-> larger batch

RTX 3050
-> QLoRA, smaller batch

CPU
-> unsupported or tiny model

============================================================
237. TRAINING PROJECT
============================================================

Persist:

model
dataset
method
hyperparameters
environment
output
metrics

============================================================
238. REPRODUCIBLE TRAINING
============================================================

Record:

- model revision
- dataset revision
- dataset hash
- code version
- runtime version
- training config
- random seed where applicable

============================================================
239. TRAINING ENVIRONMENT
============================================================

Use isolated environments.

Never corrupt system Python.

============================================================
240. TRAINING LOGS
============================================================

Stream logs into UI.

Allow export.

============================================================
241. TRAINING GPU MEMORY
============================================================

Monitor and warn.

Do not claim exact prediction if only estimated.

============================================================
242. TRAINING STOP CONDITIONS
============================================================

Allow:

- manual stop
- max steps
- max epochs
- time budget

============================================================
243. EVALUATION
============================================================

After training:

offer evaluation.

For language models:

small evaluation prompts.

For ASR:

CER/WER where reference data exists.

For TTS:

technical metrics where available.

Never fabricate evaluation scores.

============================================================
244. MODEL REGISTRATION
============================================================

Successful training output can become a local model entry.

============================================================
245. PUBLISH TRAINED MODEL
============================================================

Optional:

Publish to Hugging Face

or

Publish to TurkmenAI Marketplace

if user has legal rights and credentials.

============================================================
246. MARKETPLACE ENTITLEMENT
============================================================

Paid model:

purchase
-> entitlement
-> download access

============================================================
247. API ENTITLEMENT
============================================================

Paid API:

purchase credits
-> provider access
-> usage deduction

============================================================
248. LOCAL ENTITLEMENT
============================================================

Free/open local models do not require account.

============================================================
249. USER ACCOUNT PRIVACY
============================================================

Keep local-only users local.

Do not force sync.

============================================================
250. ORGANIZATION BILLING
============================================================

Organization can have:

- wallet
- projects
- budgets
- members

============================================================
251. BUDGET ALERTS
============================================================

Notify at:

50%
80%
100%

Configurable.

============================================================
252. COST PREVIEW
============================================================

Before a paid remote request where the provider exposes reliable pricing:

show estimated cost.

Do not guarantee exact amount if provider pricing is variable.

============================================================
253. USAGE EXPORT
============================================================

CSV/JSON export.

============================================================
254. INVOICE
============================================================

Where payment provider supports invoices:

show/download them.

============================================================
255. PROVIDER PAYOUT
============================================================

Keep financial logic server-side.

============================================================
256. FINANCIAL LEDGER
============================================================

Immutable ledger.

Use transaction IDs.

============================================================
257. PAYMENT WEBHOOK SECURITY
============================================================

Verify signatures.

Do not trust browser payment state.

============================================================
258. REPLAY PROTECTION
============================================================

Webhook events must be idempotent.

============================================================
259. ADMIN AUDIT
============================================================

All financial/admin mutations have audit records.

============================================================
260. DATABASE BACKEND
============================================================

If public marketplace backend is implemented:

PostgreSQL recommended.

============================================================
261. OBJECT STORAGE
============================================================

Use S3-compatible storage for public assets when needed.

============================================================
262. JOB QUEUE
============================================================

Use a robust background queue where needed.

============================================================
263. OBSERVABILITY
============================================================

Implement:

- structured logs
- metrics
- request IDs
- health endpoints
- provider health
- job health

Do not log prompts by default.

============================================================
264. HEALTH ENDPOINTS
============================================================

Public backend:

/health
/ready

Internal metrics should be protected.

============================================================
265. RATE LIMITING
============================================================

Apply:

- user
- API key
- project
- provider
- IP as appropriate

============================================================
266. DOS PROTECTION
============================================================

Limit:

- request body size
- file upload size
- model publish size
- concurrent jobs

============================================================
267. FILE UPLOAD SECURITY
============================================================

Scan/validate:

- archives
- dataset packages
- model metadata

Do not execute uploads.

============================================================
268. PROVIDER ENDPOINT SECURITY
============================================================

Do not store provider API keys in plaintext.

============================================================
269. MODEL SECURITY
============================================================

Custom code requires warning and isolation.

============================================================
270. DATASET SECURITY
============================================================

Treat dataset files as untrusted.

Never execute dataset code.

============================================================
271. PUBLIC API KEY FORMAT
============================================================

Use opaque random high-entropy keys.

Prefix for usability.

Store hash.

============================================================
272. KEY LAST-USED
============================================================

Track:

created
last used
revoked

============================================================
273. PROVIDER KEY ROTATION
============================================================

Allow rotate.

============================================================
274. WEBHOOK SIGNING
============================================================

HMAC or equivalent signing.

============================================================
275. PUBLIC API DOCUMENTATION
============================================================

Make docs highly professional.

Include:

Quickstart
Authentication
Responses
Chat Completions
Models
Embeddings
Tools
Structured Outputs
Streaming
Errors
Usage
Rate Limits
Billing
Providers

============================================================
276. SDK GENERATION
============================================================

If practical generate client SDK definitions from OpenAPI.

At least maintain excellent Python/JS examples.

============================================================
277. OPENAI SDK COMPATIBILITY
============================================================

Test:

OpenAI Python client

with:

base_url = TurkmenAI endpoint

using local and external compatible providers where supported.

============================================================
278. API COMPATIBILITY TEST MATRIX
============================================================

Test:

- auth
- models
- chat
- responses
- streaming
- tools
- JSON schema
- embeddings
- errors
- rate limit
- API key

============================================================
279. ERROR COMPATIBILITY
============================================================

Use predictable OpenAI-style error structures.

============================================================
280. REQUEST TRACING
============================================================

Each request gets a request ID.

Providers receive a safe propagated ID where appropriate.

============================================================
281. PROVIDER FALLBACK
============================================================

Retry/fallback only when failure is safe.

Never duplicate a non-idempotent operation accidentally.

============================================================
282. STREAM INTERRUPTION
============================================================

Handle disconnected clients without leaking backend processes.

============================================================
283. BACKGROUND API JOBS
============================================================

For large jobs:

create job ID.

Client polls or subscribes.

============================================================
284. BATCH
============================================================

Support batch concepts for:

embeddings
evaluation
dataset preprocessing
bulk inference

where backend supports it.

============================================================
285. MODEL EVALUATION
============================================================

Allow users to compare models:

quality
speed
cost
hardware
Turkmen capability

============================================================
286. MODEL COMPARISON
============================================================

Side-by-side:

- VRAM
- RAM
- size
- speed
- cost
- capability
- context
- language

============================================================
287. DATASET COMPARISON
============================================================

Side-by-side:

- size
- samples
- language
- modality
- license
- training fit

============================================================
288. TRAINING PLAN COMPARISON
============================================================

Example:

Full FT
LoRA
QLoRA

with:

VRAM
RAM
expected complexity
quality tradeoff

============================================================
289. SMART DEFAULT
============================================================

For ordinary consumer hardware:

prefer QLoRA/LoRA where full training is unnecessary.

============================================================
290. HARDWARE RESERVATION
============================================================

Before starting training:

reserve resources.

Do not start training if model inference currently consumes the same GPU unless user explicitly allows it.

============================================================
291. JOB SCHEDULER
============================================================

Unified scheduler:

downloads
quantization
benchmarks
training

Priorities:

interactive inference
voice
chat
training
background conversion

============================================================
292. TRAINING AND DOWNLOAD
============================================================

Do not run huge background tasks concurrently if they would make the system
unusable.

============================================================
293. USER EXPERIENCE
============================================================

Every expensive operation shows:

what
why
size
resources
progress
ETA
cancel

============================================================
294. WEBSITE ACCOUNT UX
============================================================

Only when needed.

Local AI:
no account.

Cloud API:
account optional/required according to billing configuration.

Marketplace:
account.

Provider:
account + verification.

============================================================
295. WEBSITE MARKETPLACE
============================================================

Build pages:

/marketplace
/marketplace/models
/marketplace/datasets
/marketplace/providers
/marketplace/apis

============================================================
296. PUBLIC MODEL PAGE
============================================================

Each model page includes:

- description
- capabilities
- compatibility
- variants
- source
- license
- price
- provider
- docs
- API
- download

============================================================
297. DATASET PAGE
============================================================

Includes:

- description
- samples
- format
- language
- size
- license
- training use
- compatibility
- download

============================================================
298. TRAINING PAGE
============================================================

Educational and interactive.

============================================================
299. PROVIDER PAGE
============================================================

Includes:

- models
- API
- pricing
- uptime
- latency
- region
- privacy

============================================================
300. SEO
============================================================

Generate indexable metadata for all public pages.

============================================================
301. SCHEMA.ORG
============================================================

Use appropriate structured data:

SoftwareApplication
Organization
Product where appropriate
Dataset where appropriate

Do not misuse schema.

============================================================
302. INTERNAL LINKS
============================================================

Model pages link to:

datasets
training guides
API
docs

Dataset pages link to:

models
training methods

Provider pages link to:

models
API docs

============================================================
303. SOCIAL CARDS
============================================================

Dynamic OG for public catalog pages.

============================================================
304. WEBSITE SECURITY
============================================================

No secrets client-side.

Strict CSP where practical.

Secure headers.

============================================================
305. ACCOUNT SECURITY
============================================================

Where accounts exist:

- passwordless or secure auth where practical
- session management
- CSRF protection
- secure cookies
- email verification when appropriate
- MFA-ready architecture

============================================================
306. PUBLIC API SECURITY
============================================================

TLS required.

No unrestricted unauthenticated inference.

============================================================
307. LOCAL API SECURITY
============================================================

Loopback by default.

============================================================
308. PAYMENT SECURITY
============================================================

Provider-hosted payment pages where possible.

Never process raw payment card information ourselves.

============================================================
309. USER DATA EXPORT
============================================================

Users can export:

- profile
- API keys metadata
- usage
- projects
- model library
- datasets
- training projects

Never export secret API keys unless the user explicitly retrieves a newly
created key.

============================================================
310. DELETE ACCOUNT
============================================================

Where account system exists:

provide deletion flow.

Handle:

- data
- billing
- entitlements
- provider accounts

according to actual platform architecture and legal requirements.

============================================================
311. SELLER DELETION
============================================================

Provider cannot delete an account while unresolved financial obligations exist.

Use safe states.

============================================================
312. MODERATION
============================================================

Create marketplace moderation.

Do not allow instant public publication of arbitrary provider content.

============================================================
313. PROVIDER API VALIDATION
============================================================

Automated smoke tests.

============================================================
314. PROVIDER PERFORMANCE
============================================================

Measure:

TTFT
throughput
error rate
timeout rate

============================================================
315. PROVIDER RANKING TRANSPARENCY
============================================================

Show why recommended:

Low latency
High reliability
Low price
etc.

============================================================
316. MONETIZATION OPTIONS
============================================================

Support configurable:

1. marketplace commission
2. API platform fee
3. premium hosted features
4. paid model/dataset sales
5. enterprise/team subscriptions
6. optional storage/compute services in future

Do not force all at launch.

============================================================
317. FREE CORE
============================================================

Local app must remain useful for free.

Do not paywall local inference.

============================================================
318. BUSINESS MODEL
============================================================

Platform revenue should come primarily from:

- marketplace transaction fees
- API infrastructure fees
- optional premium cloud services

not from locking the basic local runtime.

============================================================
319. SELLER INCENTIVE
============================================================

Providers should see:

Requests
Revenue
Conversion
Quality
Latency

============================================================
320. MODEL AUTHOR INCENTIVE
============================================================

Model authors can:

- publish
- advertise
- connect hosted API
- sell access
- receive revenue where permitted

============================================================
321. DATASET AUTHORS
============================================================

Same concept.

============================================================
322. API PROVIDER
============================================================

A user with a GPU/server can become a provider.

Flow:

Install runtime
↓
Register server
↓
Health check
↓
Set model
↓
Set price
↓
Publish

============================================================
323. SELF-HOSTED PROVIDER
============================================================

Provider endpoint should be able to run:

OpenAI-compatible server

and be registered in the marketplace.

============================================================
324. LOCAL PROVIDER
============================================================

A desktop user can expose model locally for their own apps without publishing.

============================================================
325. PROVIDER DEPLOYMENT
============================================================

Do not force TurkmenAI to host provider compute.

Providers can bring their own infrastructure.

============================================================
326. MARKETPLACE PROVIDER CONTRACT
============================================================

Define:

GET /v1/models
POST /v1/responses
POST /v1/chat/completions

Preferred:

streaming
usage
health
capabilities

============================================================
327. MODEL PRICING FORMAT
============================================================

Provider declares normalized pricing:

input per million tokens
output per million tokens
cached input where supported
request fee
image/audio units

Use Decimal/integer internal representation.

============================================================
328. COST ESTIMATOR
============================================================

Before a paid API request:

estimate cost where possible.

After:

record actual usage.

============================================================
329. PROVIDER QUOTA
============================================================

Provider can set:

- rate limit
- concurrency
- daily limit

============================================================
330. CUSTOMER QUOTA
============================================================

Customer can set budgets.

============================================================
331. ORGANIZATION QUOTA
============================================================

Same.

============================================================
332. API MARKETPLACE SEARCH
============================================================

Search:

"fast cheap coding model"

and filter providers.

============================================================
333. MODEL SEARCH INTELLIGENCE
============================================================

User can type natural language:

"самая умная туркменская модель для моей RTX 3050"

System returns ranked compatible models.

Use deterministic filters first.

Do not require cloud LLM to make basic recommendations.

============================================================
334. TRAINING NATURAL LANGUAGE
============================================================

User can say:

"Хочу дообучить эту модель на моём датасете"

System creates:

training plan

based on selected resources.

============================================================
335. DATASET NATURAL LANGUAGE
============================================================

User can say:

"Найди туркменский датасет для ASR который потянет мой компьютер"

Filter by:

task
language
hardware
size
license

============================================================
336. RESOURCE BUNDLE
============================================================

Provide:

Model
Dataset
Training Plan

as one recommendation.

============================================================
337. AI WORKSPACE
============================================================

Create workspace:

My Turkmen ASR Project

Contains:

Model
Dataset
Training
Outputs
API
Notes

============================================================
338. WORKSPACE PORTABILITY
============================================================

Export workspace metadata.

Another computer adapts hardware-specific configuration.

============================================================
339. REPRODUCIBILITY
============================================================

Record:

model revision
dataset revision
training config
runtime
code version

============================================================
340. EXPERIMENT TRACKING
============================================================

Training runs:

Run 1
Run 2
Run 3

Compare:

loss
accuracy
CER/WER
config
duration

Only available metrics.

============================================================
341. MODEL EVALUATION
============================================================

Allow manual/local evaluations.

============================================================
342. DATASET VERSIONING
============================================================

Hash source/derived dataset.

============================================================
343. DATASET DEDUP
============================================================

Where possible, deduplicate.

============================================================
344. DATASET STREAMING
============================================================

Where a training framework supports streaming, allow it to avoid storing full
datasets where feasible.

This is particularly valuable for low-disk systems.

============================================================
345. DISK ADVISOR
============================================================

Training plan must consider:

dataset
cache
checkpoints
optimizer states
temporary files

because training storage can be much larger than dataset size.

============================================================
346. TRAINING STORAGE ESTIMATION
============================================================

Give a realistic warning:

"Dataset is 4GB, but this training configuration may need significantly more
disk because of cache/checkpoints/optimizer state."

============================================================
347. CHECKPOINT POLICY
============================================================

Allow:

- save every N steps
- retain last N
- best checkpoint
- manual checkpoint

============================================================
348. AUTO CLEAN TRAINING CACHE
============================================================

Only delete files explicitly identified as disposable.

Never delete final checkpoints automatically.

============================================================
349. MODEL OUTPUT FORMATS
============================================================

Register compatible output:

- LoRA
- adapter
- merged model
- GGUF if conversion is supported

============================================================
350. MERGE SAFETY
============================================================

Never merge adapters destructively without explicit user action.

============================================================
351. PUBLISH TRAINING RESULT
============================================================

One click:

Publish to Hugging Face

or marketplace.

Check license and source rights first.

============================================================
352. OFFLINE TRAINING
============================================================

Training must work without Internet once dependencies/models/datasets are
available.

============================================================
353. TRAINING DOWNLOAD PLAN
============================================================

Before starting training:

show exactly what will be downloaded.

============================================================
354. PROGRESSIVE TRAINING ENVIRONMENT
============================================================

Do not download every framework.

Install only what's necessary for the selected job.

============================================================
355. ENVIRONMENT ISOLATION
============================================================

Use isolated runtimes.

============================================================
356. RUNTIME REUSE
============================================================

If an installed training environment already satisfies the job:

reuse it.

============================================================
357. DATASET DOWNLOAD RESUME
============================================================

Same robust engine as models.

============================================================
358. UNIVERSAL ASSET MANAGER
============================================================

Unify:

Models
Datasets
Runtimes
Training Artifacts
Packages

under one content-aware storage layer.

============================================================
359. STORAGE DEDUPLICATION
============================================================

Shared blobs across:

- models
- datasets
- packages

where byte-identical.

============================================================
360. RELEASE QUALITY
============================================================

At the end the product should include:

Desktop
CLI
Website
Public API backend
Marketplace frontend
Provider console
Training advisor
Dataset manager
Model manager
Downloads
Billing abstraction
Documentation
CI/CD

Only mark modules operational when tested.

============================================================
361. PUBLIC BACKEND IS OPTIONAL DEPLOYMENT
============================================================

Do not force every desktop installation to run:

PostgreSQL
Redis
payment server
marketplace server

Desktop remains lightweight.

Public infrastructure is separate.

============================================================
362. LOCAL DEPLOYMENT
============================================================

A developer can run:

Desktop
Local Core
Local API

without cloud.

============================================================
363. SERVER DEPLOYMENT
============================================================

Separate server stack:

API Gateway
Auth
Database
Billing
Marketplace
Provider registry
Usage service

============================================================
364. DEPLOYMENT DOCUMENTATION
============================================================

Create:

DEPLOYMENT.md

with:

- local development
- production server
- environment variables
- database
- object storage
- payment provider
- secrets
- DNS
- SSL
- workers

============================================================
365. ENVIRONMENT VARIABLES
============================================================

Create:

.env.example

No secrets.

============================================================
366. CONFIG VALIDATION
============================================================

Application should fail early with clear errors for missing production secrets.

============================================================
367. SECRET MANAGEMENT
============================================================

Never store production secrets in source.

============================================================
368. PUBLIC HEALTH
============================================================

Expose only safe:

/health
/ready

Protect metrics/admin.

============================================================
369. OBSERVABILITY
============================================================

Recommended:

- OpenTelemetry-compatible tracing
- structured logs
- metrics

Do not collect prompts by default.

============================================================
370. INCIDENT READINESS
============================================================

Document:

- rollback
- provider outage
- database backup
- payment webhook recovery
- model registry failure

============================================================
371. BACKUPS
============================================================

Public backend:

automated database backups.

Marketplace ledger must have strong durability.

============================================================
372. PAYMENT DATA RECOVERY
============================================================

Never reconstruct money state purely from UI.

Ledger is source of truth.

============================================================
373. DISASTER RECOVERY
============================================================

Document:

restore DB
restore object storage metadata
restore provider registry
restore configuration

============================================================
374. ADMIN RECOVERY
============================================================

Secure admin recovery process.

============================================================
375. API VERSION MIGRATION
============================================================

Do not break old clients.

============================================================
376. DEPRECATION SYSTEM
============================================================

Models/providers/API features can be:

active
deprecated
retired

Show migration path.

============================================================
377. MODEL RETIREMENT
============================================================

If a model disappears:

keep installed local models working.

Remote listing can show:

Deprecated
Unavailable

============================================================
378. PROVIDER RETIREMENT
============================================================

If provider goes offline:

existing entitlements should not cause silent failures.

Explain status.

============================================================
379. MARKETPLACE AVAILABILITY
============================================================

Search results should show:

Available
Degraded
Offline

============================================================
380. OFFLINE DESKTOP
============================================================

Marketplace may be unavailable offline, but local AI must continue working.

============================================================
381. FINAL DESKTOP UX
============================================================

Normal user sees:

Home
Models
Datasets
Training
Chat

Developer sees:

API
Projects
Providers
Usage

Seller sees:

Provider Dashboard
Listings
Revenue

Admin sees:

Admin Console

Use role-aware navigation.

============================================================
382. FINAL API PLATFORM UX
============================================================

The developer can:

1. choose provider
2. choose model
3. create API key
4. copy OpenAI-compatible code
5. make request
6. see usage
7. manage budget

============================================================
383. FINAL SELLER UX
============================================================

A provider can:

1. connect endpoint
2. test
3. publish
4. define price
5. receive requests
6. see revenue
7. receive payouts subject to payment provider rules

============================================================
384. FINAL TRAINING UX
============================================================

A user can:

1. select model
2. select dataset
3. inspect compatibility
4. choose training method
5. generate plan
6. download
7. train
8. evaluate
9. register result

============================================================
385. FINAL MARKETPLACE UX
============================================================

A customer can:

1. search
2. filter
3. compare
4. try
5. buy
6. use/download

============================================================
386. FINAL PRODUCT POSITIONING
============================================================

The product should now be understood as:

# TurkmenAI Platform

not only:

TurkmenAI Local.

But preserve:

TurkmenAI Local

as the desktop local runtime product.

============================================================
387. FINAL PUBLIC SITE STRUCTURE
============================================================

Home
Models
Datasets
Training
API
Marketplace
Providers
Developers
Docs
Download
Pricing
Privacy
Security

Do not make pricing a central focus for local users.

============================================================
388. PRICING PAGE
============================================================

Explain:

Local:
Free

API:
Provider pricing

Marketplace:
Item pricing

Platform fees:
where applicable

Do not invent prices.

============================================================
389. PROVIDER PRICING PAGE
============================================================

Provider controls own pricing.

============================================================
390. PLATFORM FEE TRANSPARENCY
============================================================

Clearly show platform commission before seller publishes paid products.

============================================================
391. BUYER FEE TRANSPARENCY
============================================================

Show total cost before purchase.

============================================================
392. REFUND POLICY
============================================================

Do not invent a universal refund policy.

Document actual deployment policy.

============================================================
393. TERMS
============================================================

Create placeholders/templates for legal review, but do not make false legal
claims.

============================================================
394. PRIVACY POLICY
============================================================

Separate:

desktop local processing
from:

cloud marketplace processing.

============================================================
395. DATA PROCESSING DISCLOSURE
============================================================

For each provider:

logging
retention
training
region

must be visible where available.

============================================================
396. SEO
============================================================

Build a serious technical content strategy.

Create useful articles on:

local AI
Turkmen AI
offline AI
quantization
QLoRA
Turkmen ASR
Turkmen TTS
local inference
weak hardware AI
dataset training

Do not generate spam.

============================================================
397. PUBLIC MODEL INDEX
============================================================

Search engines should be able to index real public models/datasets/providers.

============================================================
398. SITEMAP
============================================================

Generate dynamic sitemap for public catalog items.

============================================================
399. INTERNAL SEARCH
============================================================

Fast and relevant.

============================================================
400. FINAL CODE QUALITY
============================================================

Strict typing.

Rust:

cargo fmt
cargo clippy
cargo test

TypeScript:

typecheck
lint
tests

Backend:

unit/integration tests.

============================================================
401. FINAL SECURITY
============================================================

Run:

- secret scan
- dependency audit
- SAST
- auth review
- rate-limit review
- payment webhook signature validation
- permission review
- archive security
- path traversal tests

============================================================
402. FINAL BUILD
============================================================

Build:

Windows x64
Windows ARM64 where possible
Linux x64
Linux ARM64 where possible
macOS ARM64 where possible
macOS x64 where possible

Use CI for unavailable local targets.

============================================================
403. FINAL DOWNLOAD RELEASE
============================================================

All real artifacts in GitHub Releases.

Website download links must point to actual artifacts.

============================================================
404. FINAL DOCUMENTATION
============================================================

Create/update:

README
ARCHITECTURE
API
MARKETPLACE
PROVIDER
TRAINING
DATASETS
HARDWARE
QUANTIZATION
OFFLINE
SECURITY
DEPLOYMENT
BILLING
CONTRIBUTING
RELEASE

============================================================
405. FINAL ACCEPTANCE — LOCAL
============================================================

A normal user must be able to:

- install
- detect hardware
- find model
- filter compatible models
- download
- resume
- run
- chat
- use offline

============================================================
406. FINAL ACCEPTANCE — DATASET
============================================================

User:

- searches dataset
- sees compatibility
- downloads
- inspects
- derives
- uses for training

============================================================
407. FINAL ACCEPTANCE — TRAINING
============================================================

User:

- chooses model
- chooses dataset
- sees feasible training methods
- creates TrainingPlan
- runs training
- sees progress
- gets checkpoint
- validates result

============================================================
408. FINAL ACCEPTANCE — API
============================================================

Developer:

- adds OpenAI-compatible API
- tests
- creates project key
- sends request
- gets streaming result
- sees usage
- sees cost

============================================================
409. FINAL ACCEPTANCE — PROVIDER
============================================================

Provider:

- signs up
- onboards payment
- connects API
- passes validation
- publishes model
- receives traffic
- sees usage
- sees revenue/payout status

Only test end-to-end payment if test credentials are available.

============================================================
410. FINAL ACCEPTANCE — MARKETPLACE
============================================================

Buyer:

- searches
- filters
- chooses provider
- sees price
- purchases
- receives entitlement
- uses API/model/dataset

============================================================
411. FINAL ACCEPTANCE — TURKMENISTAN INTERNET
============================================================

At simulated 3–4 MB/s:

- small model should start before large optional downloads finish
- downloads resume after disconnect
- dataset downloads resume
- large files do not restart
- bandwidth can be limited
- queue remains usable

============================================================
412. FINAL PRODUCT DIFFERENTIATOR
============================================================

The most important unique experience must be:

"What can I actually do with THIS computer?"

TurkmenAI should answer:

Models I can run
Datasets I can use
Training I can perform
APIs I can connect to
AI stack I should install

============================================================
413. FINAL SYSTEM DIAGRAM
============================================================

                   TURKMENAI PLATFORM
                            |
          +-----------------+-----------------+
          |                 |                 |
        LOCAL             MARKETPLACE       API PLATFORM
          |                 |                 |
      Models            Models/Datasets    Providers
      Datasets          APIs              Routing
      Training          Sellers           Usage
      Chat              Payments           Billing
      Runtime           Payouts            API Keys
          |                 |                 |
          +-----------------+-----------------+
                            |
                    UNIVERSAL CORE/API
                            |
        +-------------------+-------------------+
        |                   |                   |
      Model               Dataset            Training
     Resolver             Resolver             Planner
        |                   |                   |
        +-------------------+-------------------+
                            |
                     HARDWARE ENGINE
                            |
                     EXECUTION PLANNER
                            |
                    +-------+-------+
                    |               |
                  Local           Remote
                    |               |
              llama.cpp/etc.   API providers
                    |               |
                    +-------+-------+
                            |
                       USER / APPS
============================================================
414. FINAL PRINCIPLE
============================================================

TurkmenAI should become the easiest place to answer:

"What AI can I use?"

"What can my PC run?"

"What can I train?"

"Where do I get the model?"

"Where do I get the dataset?"

"How do I expose it as an API?"

"How much will the API cost?"

"Can I sell my own AI?"

And all of these should be solved through ONE coherent platform.

============================================================
415. DEVELOPMENT PRIORITY
============================================================

Do not implement everything at once.

Prioritize in this order:

P0:
Existing app reliability

P1:
Model + dataset + hardware system

P2:
Training Advisor

P3:
Unified API / provider abstraction

P4:
API console

P5:
Marketplace catalog

P6:
Provider publishing

P7:
Billing/payments

P8:
SEO/public catalog

P9:
Advanced ecosystem features

Do not spend time on P9 while P0-P4 are broken.

============================================================
416. FINAL INSTRUCTION
============================================================

Continue from the CURRENT repository.

Do not restart.

Do not delete functioning work.

Do not produce another conceptual plan.

Implement.

Test.

Refactor.

Integrate.

Document.

Build.

Verify.

The final platform must have a coherent architecture and must not become a
random collection of unrelated projects.

Use upstream software wherever it already solves the problem.

Write original code primarily for:

- orchestration
- compatibility
- hardware intelligence
- dataset/training intelligence
- provider abstraction
- API gateway
- marketplace
- billing integration
- Turkmenistan-specific networking
- security
- UX

Do not write your own payment processor.
Do not write your own inference engine.
Do not write your own quantization mathematics.
Do not write your own KYC system.
Do not write your own model hosting protocol unnecessarily.

Use mature infrastructure.

============================================================
417. FINAL DELIVERABLE
============================================================

The final product must be:

- production-grade
- installable
- cross-platform where supported
- testable
- documented
- secure
- offline-first
- model-aware
- dataset-aware
- training-aware
- API-compatible
- marketplace-ready
- monetization-ready
- developer-friendly

Final reports must clearly distinguish:

IMPLEMENTED
TESTED
PARTIAL
EXPERIMENTAL
BLOCKED

Never fake completion.

============================================================
418. START NOW
============================================================

Inspect the current repository.

Create the architecture audit.

Then implement the highest-priority missing capabilities.

Do not stop at the first successful build.

Continue through testing, integration, polish, and production validation.

END OF PLATFORM 2.0 DIRECTIVE
```

[1]: https://docs.litellm.ai/?utm_source=chatgpt.com "LiteLLM - Getting Started | liteLLM"
[2]: https://openrouter.ai/providers/apply?utm_source=chatgpt.com "Become a Provider | OpenRouter"
[3]: https://docs.stripe.com/connect/marketplace?utm_source=chatgpt.com "Build a marketplace | Stripe Documentation"
[4]: https://platform.openai.com/docs/api-reference/responses-streaming/response/refusal/delta?lang=curl&utm_source=chatgpt.com "Streaming events | OpenAI API Reference"
[5]: https://docs.stripe.com/connect/marketplace/tasks/create?utm_source=chatgpt.com "Create and prefill the connected account | Stripe Documentation"
[6]: https://docs.stripe.com/connect/embedded-onboarding?locale=en-GB&utm_source=chatgpt.com "Embedded onboarding | Stripe Documentation"
Да, **технически такую систему построить можно**, и твоя идея очень похожа на нормальный prepaid/usage-billing слой: пользователь пополняет баланс, получает кредиты, использует API, кредиты списываются, а заработок поставщика накапливается в отдельном ledger.

Но есть важный нюанс: **не надо строить это как “все деньги просто лежат на моём одном Bybit-аккаунте, а я внутри базы рисую балансы” без продуманной финансовой и юридической модели**. В таком виде ты фактически становишься посредником, который принимает средства пользователей, ведёт их внутренние балансы и выплачивает третьим лицам. Это может иметь серьёзные требования по платежам/криптоактивам/AML/KYC в зависимости от юрисдикции.

Технически Bybit действительно предоставляет API для проверки on-chain депозитов, транзакционного журнала и withdrawals. Например, API депозитов возвращает `txID`, coin, chain, amount, address, confirmations и статус; withdrawal API позволяет создавать вывод, причём для него нужны соответствующие полномочия и whitelist-адреса. ([Bybit Exchange][1])

### Я бы построил TurkmenAI Billing вот так

```text
                  USER
                    │
                    ▼
             TurkmenAI Wallet
                    │
             "Add $10 / $25"
                    │
                    ▼
             Payment / Crypto
                    │
                    ▼
              BYBIT TREASURY
                    │
              Deposit Monitor
                    │
                    ▼
            Payment Reconciliation
                    │
                    ▼
             Internal Ledger
                    │
          +---------+---------+
          │                   │
          ▼                   ▼
      User Credits        Platform Revenue
          │
          ▼
      API Request
          │
          ▼
      Usage Meter
          │
          ▼
    1000 tokens = X credits
          │
          ▼
     Provider Ledger
          │
          ▼
   Provider Withdraw Request
          │
          ▼
   Manual / Automated Payout
```

## Главное: не делай `balance = 10`

Нужен **immutable ledger**.

Например:

```text
User #42

+10.00  deposit
-0.021  API usage
-0.140  API usage
+5.00   refund
----------------
14.839 credits
```

А отдельно:

```text
Provider #17

+0.080  API usage
+1.240  API usage
-0.500  payout
----------------
0.820 available
```

То есть:

**баланс — это производное значение ledger, а не единственная цифра в таблице.**

---

# Как работает пополнение

Пользователь выбирает:

```text
Add funds

$5
$10
$25
$50
Custom
```

Дальше:

```text
Choose asset:
USDT

Choose network:
TRC20
```

и получает инструкцию.

Но тут есть **критическая проблема**.

У Bybit мастер-аккаунта депозитный адрес может быть общим для конкретной монеты/сети; API позволяет получить master deposit address, а записи депозитов содержат сумму, txID и адрес назначения. ([Bybit Exchange][2])

Поэтому **нельзя надёжно определить пользователя просто по адресу**, если все платят на один и тот же адрес.

---

# Поэтому нужны Payment Intents

Когда пользователь нажимает:

**Пополнить $10**

создаётся:

```text
Payment Intent

id:
pay_8f7...

user:
user_42

expected:
10 USDT

currency:
USDT

network:
TRC20

expires:
30 min

status:
waiting
```

И дальше система пытается сопоставить платёж.

### Но не делай тупой алгоритм:

> «Пришло 10 USDT → дать деньги тому, кто сейчас открыл страницу».

Это небезопасно.

Нужна нормальная reconciliation-система.

---

# Лучший вариант с твоим Bybit

На первом этапе:

```text
User
 ↓
Payment Intent
 ↓
Unique payment reference / supported attribution mechanism
 ↓
Bybit
 ↓
Deposit Monitor
 ↓
Match transaction
 ↓
Confirmations
 ↓
Ledger
 ↓
Credits
```

Bybit API позволяет получать историю on-chain deposits и их `txID`, amount, chain, address, confirmations и статус, что подходит для автоматического reconciliation. ([Bybit Exchange][1])

Но если выбранный способ депозита **не даёт надёжного способа идентифицировать конкретного пользователя**, не зачисляй автоматически только по совпадению суммы.

В таком случае:

```text
Payment detected
↓
Needs verification
```

и пользователь указывает TXID, после чего система проверяет его по Bybit.

---

# Ещё лучше — отдельная payment abstraction

В коде вообще не должно быть:

```text
BybitPaymentService
```

внутри всего приложения.

Сделай:

```text
PaymentProvider
```

и:

```text
PaymentProvider
├── BybitCryptoProvider
├── StripeProvider
├── PaddleProvider
├── LemonSqueezyProvider
└── FutureCryptoProvider
```

Тогда сегодня:

```text
Bybit
```

завтра:

```text
другой crypto gateway
```

а потом:

```text
карты
Apple Pay
Google Pay
```

без переписывания billing.

---

# А теперь самое интересное — кредиты

Я бы **не делал 1 кредит = $1 напрямую в базе**, хотя UI может это показывать.

Внутри:

```text
USD minor units
```

или decimal-safe currency.

Например:

```text
$10.00
```

→

```text
1000 cents
```

И API usage списывает определённую стоимость.

---

# Модель ценообразования

Например:

### TurkmenAI Fast 7B

```text
Input:
$0.05 / 1M tokens

Output:
$0.20 / 1M tokens
```

### TurkmenAI Pro 32B

```text
Input:
$0.40 / 1M

Output:
$1.20 / 1M
```

### Turkmen ASR

```text
$0.003 / minute
```

### TTS

```text
$0.01 / 1000 characters
```

### Image

```text
$0.02 / image
```

Но лучше сделать **универсальную pricing engine**:

```text
PricingMetric

tokens
characters
seconds
images
requests
compute_seconds
```

и тариф:

```text
input_tokens
output_tokens
cached_input_tokens
audio_seconds
image_count
request_fee
```

Так система сможет поддерживать практически любой AI API.

---

# Тогда пользователь видит

```text
Balance

$12.48

This month:
$3.71

Estimated remaining:
~2,800 requests
```

А после запроса:

```text
TurkmenAI 8B

Input:
1,823 tokens

Output:
741 tokens

Cost:
$0.00042
```

---

# А поставщик получает свои деньги

Допустим:

Пользователь заплатил:

```text
$10
```

Провайдер получает:

```text
$7.50
```

TurkmenAI:

```text
$2.50
```

Но **не надо сразу реально переводить эти $7.50 каждый раз**.

Наоборот:

```text
Provider Ledger

Pending:
$7.50
```

После выполнения условий:

```text
Available:
$7.50
```

И продавец нажимает:

**Withdraw**

---

# Withdrawal system

```text
Provider Balance

Pending
Available
Paid
```

Запрос:

```text
Withdraw $100

Destination:
USDT TRC20
Address:
T...
```

Сначала:

```text
PENDING_REVIEW
```

Потом:

```text
APPROVED
```

и уже затем:

```text
PAID
```

---

# Я бы на старте НЕ делал автоматический вывод

Это очень важно.

Для первой версии:

```text
Provider → Withdraw Request → YOU REVIEW → PAY
```

То есть продавец видит:

> Withdrawal requested — $27.40

А у тебя в Admin:

```text
Withdrawal #182

Provider:
...

Amount:
27.40 USDT

Network:
TRC20

Destination:
...

Status:
Pending approval

[Approve]
[Reject]
```

После ручного перевода:

```text
Mark as Paid
TXID:
...
```

Это **намного безопаснее**, пока у тебя нет нормальной compliance/финансовой инфраструктуры.

---

# Почему я не советую сразу автоматический withdrawal

Потому что Bybit withdrawal API действительно позволяет создавать вывод через API, но это уже очень чувствительная операция; документация указывает необходимость master-account permissions и рекомендует whitelist адресов. ([Bybit Exchange][3])

Если украдут твой ключ с withdrawal permission:

```text
API
↓
Bybit
↓
все деньги
```

могут оказаться под угрозой.

Поэтому архитектурно:

```text
READ API KEY
```

для:

* deposit monitoring;
* balances;
* transaction logs;

и **отдельный высокозащищённый withdrawal credential**, вообще не используемый обычным сервером.

А на первом этапе withdrawal — ручной.

---

# А ещё я бы использовал subaccounts

У Bybit есть API для создания sub UID и управления ими, а также внутренние переводы между счетами. ([Bybit Exchange][4])

Теоретически можно сделать:

```text
TurkmenAI Treasury
│
├── Provider A
├── Provider B
├── Provider C
└── ...
```

Но **я не рекомендую автоматически создавать Bybit subaccount каждому пользователю на старте**. Это усложнит KYC/compliance, управление и операции.

Subaccounts могут быть полезнее для **provider treasury / operational segregation**, если текущие условия Bybit это допускают для твоего типа аккаунта.

---

# Более правильная финансовая архитектура

Тебе нужны четыре независимых слоя:

```text
PAYMENTS
   ↓
LEDGER
   ↓
ENTITLEMENTS
   ↓
USAGE
```

Например:

### Payments

```text
$10 received
```

### Ledger

```text
+1000 cents
```

### Entitlement

```text
1000 credits available
```

### Usage

```text
-17 credits
```

Никогда не смешивай эти четыре вещи.

---

# И самое важное: деньги ≠ кредиты

Например:

Пользователь внёс:

```text
$10
```

Получил:

```text
1000 credits
```

Потом ты можешь изменить promotional bonus:

```text
$10 → 1100 credits
```

Но financial ledger всё равно говорит:

```text
$10 received
```

Это делает систему гораздо более нормальной для дальнейшего масштабирования.

---

# Marketplace тогда выглядит очень круто

### TurkmenGPT

```text
Provider: TurkmenAI

Input
$0.05 / 1M tokens

Output
$0.20 / 1M tokens

Context
32K

Latency
Low

[Try]
[Use API]
```

### Другой продавец

```text
Provider: Ashgabat AI

Turkmen LLM Pro

Input
$0.02 / 1M

Output
$0.08 / 1M

[Try]
```

Пользователь вообще не думает:

> где чей сервер?

Он просто выбирает модель.

---

# А внутри TurkmenAI

Router решает:

```text
Request
 ↓
Model ID
 ↓
Provider
 ↓
Price
 ↓
Health
 ↓
Privacy
 ↓
Route
```

То есть твоя система становится чем-то средним между:

**OpenAI API + OpenRouter + Hugging Face + LM Studio/Ollama + marketplace.**

---

# Но я бы добавил ещё одну функцию

### `Bring Your Own Key`

Пользователь может сказать:

> У меня уже есть OpenAI API key.

Вставляет:

```text
OpenAI
sk-...
```

и получает:

```text
OpenAI
GPT...
✓ Connected
```

Без пополнения TurkmenAI.

То же самое для любого OpenAI-compatible API.

И тогда TurkmenAI становится **универсальным AI console**, а не только продавцом своих API.

---

# А для тебя — самый безопасный старт

Я бы сделал три режима:

### 1. LOCAL

```text
$0
```

Полностью локально.

### 2. BYOK

```text
$0 TurkmenAI fee
```

Пользователь использует собственный API key.

### 3. TURKMENAI CREDIT

```text
Пополняешь баланс
↓
используешь marketplace APIs
↓
автоматическое списание
```

А потом:

### 4. SELLER

Разработчик подключает свой API и продаёт доступ.

---

## Но есть юридическая граница

Если ты принимаешь деньги/криптоактивы от пользователей, превращаешь их в внутренние кредиты, учитываешь остатки, удерживаешь средства продавцов и затем выплачиваешь продавцам, это уже **не просто “API billing”**. В зависимости от страны, структуры и используемого актива это может затрагивать регулирование платежей, электронных денег/предоплаченных средств, криптоактивов, AML/KYC и marketplace payments.

Поэтому **технически систему можно спроектировать именно так, но запускать реальный money flow нужно только после проверки юрисдикции и условий Bybit/платёжного провайдера**. Особенно не стоит строить схему, которая скрывает от провайдера, что ты принимаешь деньги за услуги третьих лиц.

Bybit действительно поддерживает автоматический просмотр депозитов и операции вывода через API, но условия аккаунта, разрешённые операции и compliance нужно проверить именно для твоего аккаунта; у Bybit также есть отдельные правила по переводам и идентификации пользователей. ([Bybit Exchange][1])

### Моя рекомендация

**Технически уже сейчас заложить полноценный `PaymentProvider + Ledger + Credits + Usage + ProviderBalance + Withdrawal`**, но на первом запуске сделать:

```text
Пополнение → автоматическая сверка → кредиты
                    ↓
                API usage
                    ↓
             Seller balance
                    ↓
             Withdraw request
                    ↓
              твоя проверка
                    ↓
               выплата
```

А автоматические payouts подключать после того, как будет подтверждено, что твоя юридическая и платёжная схема это позволяет.

И отдельный важный момент: **никогда не давай Claude/Manus полный Bybit API key с `Withdrawal` и торговыми правами для разработки**. Для первого этапа достаточно read-only доступа для проверки депозитов/истории; withdrawal должен быть отдельным, изолированным контуром. Bybit позволяет ограничивать permissions API-ключей, а некоторые операции, включая withdrawals, требуют повышенных прав. ([Bybit Exchange][5])

[1]: https://bybit-exchange.github.io/docs/v5/asset/deposit/deposit-record?utm_source=chatgpt.com "Get Deposit Records (on-chain) | Bybit API Documentation"
[2]: https://bybit-exchange.github.io/docs/v5/asset/deposit/master-deposit-addr?utm_source=chatgpt.com "Get Master Deposit Address | Bybit API Documentation"
[3]: https://bybit-exchange.github.io/docs/v5/asset/withdraw?utm_source=chatgpt.com "Withdraw | Bybit API Documentation"
[4]: https://bybit-exchange.github.io/docs/v5/user/create-subuid?utm_source=chatgpt.com "Create Sub UID | Bybit API Documentation"
[5]: https://bybit-exchange.github.io/docs/v5/user/create-subuid-apikey?utm_source=chatgpt.com "Create Sub UID API Key | Bybit API Documentation"
