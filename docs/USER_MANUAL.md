# User Manual

TurkmenAI Local is designed around a simple principle: you should understand what will run on your computer before it runs. The desktop interface offers a public product overview and a local console. The local console does not pretend to be connected when it is opened in an ordinary browser; it shows real machine data only when loaded by the native desktop shell.

## First launch

Install a verified package for your operating system, then start **TurkmenAI Local**. The application uses the local machine by default. It does not need a cloud account to show the architecture, inspect the machine profile, or create a plan.

| Action | What happens locally |
|---|---|
| Open **Local console** | The desktop shell asks the Core for the actual OS, processor, RAM, disk and accelerator profile. |
| Open a model source | The resolver reads paths and metadata. It does not execute source scripts. |
| Request an execution plan | The planner compares model requirements, backend capabilities and hardware limits. |
| See `requires_code` | Treat it as a warning. Do not assume the model can be safely launched. |
| Start API mode | Run `tmai server 8742`; only software on the same machine can reach it by default. |

## Understanding plans

Every plan identifies an artifact type, backend, context limit, offload strategy, approximate memory needs, score and evidence type. **Estimated** means the figure is calculated from available metadata. A later version may show **measured** only after a controlled benchmark is recorded locally.

When a plan does not fit, TurkmenAI Local first proposes reversible actions such as a lower context length, different offload strategy or a pre-quantized artifact. It does not rewrite original model files or silently download a different model.

## Using the local API

Start the service with `tmai server`. It listens on `http://127.0.0.1:8742` only. You can inspect service state with:

```bash
curl http://127.0.0.1:8742/api/v1/health
```

The inference route is intentionally unavailable until a runtime has been explicitly installed, verified and connected. A `NO_ACTIVE_RUNTIME` response is a correct safety state, not an instruction to run an untrusted script.

## If something is not available

The product does not include a bundled LLM, an automatic `trust_remote_code` route, public sharing, cloud fallback or fake “successful” responses. Consult [Troubleshooting](TROUBLESHOOTING.md) for practical local checks.
