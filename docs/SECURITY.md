# Security Model

TurkmenAI Local treats model repositories, metadata, archives, README files and custom URLs as untrusted inputs. Resolver classification is conservative: scripts and executable files cause an explicit risk state, and no code from a model source is executed automatically.

The default trust boundary is local. Telemetry, cloud inference, automatic uploads and LAN sharing are off. The local API binds to `127.0.0.1`; an eventual LAN mode must use a generated API key, user confirmation and explicit firewall guidance. Model blob paths are content-addressed by SHA-256 and copied atomically after verification.

External runtimes run as explicitly configured child processes in a dedicated workspace. They do not inherit arbitrary model repository scripts. The current supervisor clears the process environment before providing only the process path and workspace home. Destructive tool calls, plugin permissions, agent capabilities and network access remain out of scope until a specific permission layer is implemented and tested.
