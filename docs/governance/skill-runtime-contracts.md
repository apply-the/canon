# Skill Runtime Contracts

Modern AI agents often rely on "Skills" or "Tools" (like MCP servers, shell execution, or browser automation) to interact with the real world. In an ungoverned environment, giving an AI a shell tool is a recipe for catastrophic failure.

Canon mitigates this by wrapping every capability in a **Skill Runtime Contract**.

## What is a Skill Contract?

A Skill Contract is a strict definition of what an agent is allowed to do with a specific tool during a specific mode. 

Instead of an agent having generic "shell access", Canon defines constraints:
- **Preflight Validation**: Before a skill is executed, Canon checks if the active Governance Packet allows this skill.
- **Arg Validation**: Canon intercepts the arguments passed by the AI (e.g., verifying that a shell command does not contain `rm -rf` or attempt to leave the bounded workspace).

## Hook Traces

Every invocation of a skill is recorded. The inputs, the raw stdout/stderr output of the tool, and the execution time are captured in **Hook Traces**.

If an agent uses a debugging skill (like Chrome DevTools for a memory leak), the full diagnostic session is persisted. This means human reviewers can see exactly what the agent "saw" when making its subsequent coding decisions.

## Enforcement by the Host

Canon itself does not execute the subprocesses—that is the job of the execution host (like Boundline). Canon's role is to **dictate the rules**. Boundline implements the V1 Adapter Protocol and consults Canon's Skill Runtime Contracts to determine if a requested tool execution is legal. If the contract is violated, Canon blocks the request instantly.