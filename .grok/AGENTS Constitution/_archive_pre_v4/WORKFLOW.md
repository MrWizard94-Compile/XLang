Workflow Stages
---------------

1. Research 10 well-known projects relevant to the current project/workspace.

2. Decompose each project into core components.

3. Study each project in depth.

4. Design a new system based on the best features of the researched projects.

5. Write foundational documentation.

6. Plan engineering.

7. Implement the engineering plan.

8. Audit the codebase and apply fixes.

9. Harden the codebase.

10. Perform a final audit/fix pass and deliver the product.

11. Research Projects

--------------------

Select 10 mature projects that solve problems close to the one you are building. The goal is to understand successful patterns, tradeoffs, and operational practices before designing your own system.itential+1

For each project, record:

* Name and purpose.

* Target users.

* Core workflows.

* Key differentiators.

* Platform or architecture style.

* Strengths, weaknesses, and gaps.
2. Decompose Components

-----------------------

Break every project into comparable pieces so you can evaluate them consistently. A useful decomposition is UI, domain logic, data layer, integrations, permissions, background jobs, reporting, and operational tooling.chainguard+2

Use this template:

* Frontend or user-facing experience.

* Core domain objects and rules.

* APIs and integrations.

* Persistence and state handling.

* Security, auth, and permissions.

* Automation and background processing.

* Observability and admin tools.
3. Deep Study

-------------

Study each project in depth to understand why it works and where it breaks down. This stage should capture not only visible features, but also workflow friction, architecture choices, and constraints that shaped the solution.equorum+1

Document for each project:

* Main user journey.

* Important screens or entry points.

* Repeated patterns and system behaviors.

* Performance or scalability clues.

* Security or compliance characteristics.

* Missing features or pain points.
4. Synthesize New Design

------------------------

Use the research to design a new system that combines the strongest ideas into a coherent product. Secure development guidance recommends that design decisions explicitly address trust boundaries, dependencies, data sensitivity, and auditability.chainguard+1

Outputs for this stage:

* Product vision.

* Feature priorities.

* User stories.

* Trust boundaries.

* Data classification.

* Threat model notes.

* Success criteria.
5. Foundational Documentation

-----------------------------

Write the project documentation before engineering begins so the team shares a single source of truth. Audit-ready workflows depend on traceability, integrity, role-based access, and standardized approval steps.[youtube](https://www.youtube.com/watch?v=6Aqo9uJAMTA)[llamaindex](https://www.llamaindex.ai/glossary/audit-ready-document-workflows)

Minimum docs:

* Product requirements document.

* Functional specifications.

* UX/workflow maps.

* Architecture overview.

* Data model and schema notes.

* API documentation.

* Security and access-control policy.

* Test and release plan.
6. Engineering Plan

-------------------

Convert the documentation into a build plan with milestones, responsibilities, and dependencies. Secure SDLC guidance recommends defining requirements, design, implementation, testing, deployment, and maintenance as explicit phases with owners and gates.aikido+1

Engineering plan should include:

* Tech stack.

* Repository and branching strategy.

* Milestone plan.

* Definition of done.

* Test strategy.

* Dependency and release controls.

* Risk register.
7. Implement Plan

-----------------

Build the product in increments, starting with the smallest usable core. During implementation, enforce secure coding standards, dependency controls, and evidence collection so the build remains auditable.chainguard+1

Recommended implementation order:

1. Core data model.

2. Authentication and authorization.

3. Primary user workflow.

4. Integrations and automation.

5. Reporting and admin tools.

6. Logging, monitoring, and release readiness.

7. Codebase Audit

-----------------

Audit the codebase after the first implementation pass to catch defects, security issues, and architecture drift. Secure SDLC guidance emphasizes controls such as automated checks, traceability, and explicit review of dependencies and sensitive paths.paloaltonetworks+2

Audit checklist:

* Functional correctness review.

* Security review.

* Dependency and supply-chain review.

* Test coverage review.

* Logging and observability review.

* Performance and reliability review.
9. Harden Codebase

------------------

Hardening is where the system is stabilized for real-world use. This includes tightening secrets handling, reducing attack surface, improving validation, and adding operational safeguards such as alerting and anomaly detection.paloaltonetworks+2

Hardening tasks:

* Remove known vulnerabilities.

* Tighten permissions and least-privilege access.

* Improve input validation and error handling.

* Add rate limiting and abuse protection.

* Strengthen logging and evidence capture.

* Improve deployment and rollback controls.
10. Final Audit and Delivery

----------------------------

Perform one final audit, fix remaining issues, and deliver the product. Audit-ready workflows rely on a clear record of changes, approvals, and final verification before release.[llamaindex](https://www.llamaindex.ai/glossary/audit-ready-document-workflows)[youtube](https://www.youtube.com/watch?v=6Aqo9uJAMTA)

Final delivery checklist:

* Re-run tests and security scans.

* Verify documentation is current.

* Confirm release and rollback readiness.

* Validate monitoring and alerting.

* Package product delivery and handoff materials.
