# GitHub Copilot Instructions for rust-zmq

## Repository Overview

This is the **rust-zmq** repository, providing Rust bindings for the ZeroMQ (libzmq) library. This crate offers safe Rust API bindings that closely follow the C API of libzmq.

### Repository Structure

```
rust-zmq/
├── build.rs                    # Build script for compilation
├── Cargo.toml                  # Main package manifest
├── src/                        # Main library source code
│   ├── lib.rs                  # Library entry point
│   ├── message.rs              # ZMQ message handling
│   └── sockopt.rs              # Socket options implementation
├── examples/                   # Example applications
│   ├── msgsend/                # Message sending example
│   ├── stream/                 # Stream processing example
│   └── zguide/                 # ZeroMQ guide examples
│       ├── helloworld_client/
│       ├── helloworld_server/
│       ├── weather_client/
│       ├── weather_server/
│       └── [many other examples]
├── tests/                      # Test suite
│   ├── compile-tests.rs        # Compilation tests
│   ├── connection.rs           # Connection tests
│   ├── context.rs              # Context tests
│   ├── message.rs              # Message tests
│   ├── common/                 # Shared test utilities
│   ├── compile-fail/           # Expected compilation failures
│   └── [other test modules]
├── zmq-sys/                    # Low-level FFI bindings
│   ├── Cargo.toml              # Sys crate manifest
│   ├── build/                  # Build utilities
│   └── src/                    # FFI declarations
├── msrv-test/                  # Minimum Rust version testing
├── CONTRIBUTING.md             # Contribution guidelines
├── README.md                   # Project documentation
├── NEWS.md                     # Release notes
└── LICENSE-*                   # License files
```

## Critical Instructions

### 🚨 File Modification Restrictions

- **DO NOT MODIFY** any `*.codegen.go` files if present in the repository
- These files are auto-generated and should never be manually edited
- If codegen files need updates, modify the source templates or generation scripts instead

### 🎯 Jira Integration Workflow

When a Jira ID is provided:
1. Use the **atlassian-mcp-server** MCP server to fetch Jira issue details
2. Read and understand the story/task requirements thoroughly
3. Implement the task according to the Jira specifications
4. Ensure implementation aligns with the issue's acceptance criteria

### 🧪 Testing Requirements

- **ALWAYS** create comprehensive unit test cases for any implementation
- Maintain test coverage **> 80%** for the repository
- Run existing tests to ensure no regressions: `cargo test`
- Add integration tests when implementing new features
- Test both success and error scenarios
- Follow the existing test structure in the `tests/` directory

### 📋 Prompt-Based Task Execution

All tasks must be performed in a prompt-based manner:
1. After each step, provide a **clear summary** of what was completed
2. State what the **next step** will be
3. List all **remaining steps** to complete the task
4. **Ask for confirmation** before proceeding to the next step
5. Wait for user approval before continuing

### 🔄 Complete Workflow for Task Implementation

Follow this comprehensive workflow when implementing any task:

#### Phase 1: Analysis & Planning
1. **Understand Requirements**
   - Read Jira issue (if provided) using atlassian-mcp-server
   - Analyze existing codebase
   - Identify affected components
   - Plan implementation approach

2. **Create Implementation Plan**
   - Break down task into smaller steps
   - Identify files to be modified/created
   - Plan test strategy
   - Estimate impact on existing functionality

#### Phase 2: Implementation
3. **Setup Development Branch**
   - Create branch named after Jira ID (if provided) or descriptive name
   - Ensure clean working directory

4. **Implement Core Changes**
   - Make necessary code changes
   - Follow Rust best practices and idioms
   - Maintain consistency with existing code style
   - Add comprehensive documentation

5. **Add/Update Tests**
   - Create unit tests for new functionality
   - Update existing tests if needed
   - Ensure test coverage > 80%
   - Run `cargo test` to verify all tests pass

#### Phase 3: Validation & Submission
6. **Quality Assurance**
   - Run `cargo clippy` for linting
   - Run `cargo fmt` for formatting
   - Check test coverage with `cargo tarpaulin` (if available)
   - Verify no prohibited files were modified

7. **Create Pull Request** (if requested)
   - Push changes to branch
   - Create PR using GitHub CLI
   - Add labels "runtest:all:stable" and "ai-assisted" to PR
   - Include comprehensive PR description with HTML formatting

8. **Update JIRA Ticket** (mandatory after PR creation)
   - Use atlassian-mcp-server to update the Jira issue
   - Set custom field customfield_11170 ("Does this Work Include AI Assisted Code?") to "Yes"
   - Use correct field format: {"customfield_11170": {"value": "Yes"}}
   - Verify the field update was successful

### 📝 Pull Request Creation Process

When prompted to create a PR:

1. **Branch Management**
   ```bash
   # Create and switch to branch (use Jira ID as branch name if provided)
   git checkout -b <jira-id-or-descriptive-name>
   
   # Add and commit changes
   git add .
   git commit -m "Brief description of changes"
   
   # Push to remote
   git push origin <branch-name>
   ```

2. **GitHub CLI Authentication**
   ```bash
   # Authenticate with GitHub (no .profile reference)
   gh auth login
   ```

3. **Create PR with GitHub CLI**
   ```bash
   # Create PR with proper labeling
   gh pr create \
     --title "Descriptive PR Title" \
     --body-file pr_description.md \
     --label "runtest:all:stable" \
     --label "ai-assisted"
   ```

4. **PR Description Format** (HTML formatted)
   ```html
   <h2>Summary</h2>
   <p>Brief overview of changes made</p>
   <p><em>This work was completed with AI assistance following Progress AI policies</em></p>
   
   <h3>Changes Made</h3>
   <ul>
     <li>Specific change 1</li>
     <li>Specific change 2</li>
   </ul>
   
   <h3>Testing</h3>
   <ul>
     <li>Test coverage: X%</li>
     <li>All existing tests pass</li>
     <li>New tests added for [specific functionality]</li>
   </ul>
   
   <h3>Jira Issue</h3>
   <p>Resolves: [Jira-ID] - [Issue Title]</p>
   ```

### 🏷️ PR Labeling

Always add the following labels to any PR created using GitHub CLI:
- **"runtest:all:stable"** - to ensure proper CI/CD pipeline execution
- **"ai-assisted"** - to indicate work completed with AI assistance following Progress AI policies

To create the ai-assisted label (run once per repository):
```bash
gh label create "ai-assisted" --color "9A4DFF" --description "Work completed with AI assistance following Progress AI policies" --force
```

### 🔧 Development Guidelines

- **Language**: All code should be written in Rust (edition 2018+)
- **Dependencies**: Follow existing dependency patterns in `Cargo.toml`
- **Documentation**: Use rustdoc comments for public APIs
- **Error Handling**: Use proper Rust error handling patterns
- **FFI Safety**: Be extra careful with unsafe code in zmq-sys bindings
- **Performance**: Consider performance implications for high-throughput scenarios
- **Compatibility**: Maintain compatibility with libzmq 4.1+

### 🚀 Local Development Setup

- All tasks are performed on the local repository
- Use standard Rust toolchain: `cargo build`, `cargo test`, `cargo clippy`
- Examples can be run with: `cargo run --example <example_name>`
- Use `cargo doc --open` to generate and view documentation

### 📊 Quality Metrics

- **Test Coverage**: > 80% (mandatory)
- **Clippy Warnings**: Zero warnings allowed
- **Documentation**: All public APIs must be documented
- **Performance**: No significant performance regressions

### 🔍 Troubleshooting

- Check existing issues and PRs before implementing
- Consult ZeroMQ documentation for protocol-specific questions  
- Review contribution guidelines in `CONTRIBUTING.md`
- Test with different libzmq versions when relevant

---

## Confirmation Protocol

Before proceeding with any step in the workflow:
1. **Present the plan** for the next action
2. **Ask**: "Would you like me to continue with the next step?"
3. **Wait for explicit confirmation** before proceeding
4. **Provide status updates** throughout the process

This ensures full transparency and control over the development process.

---

## 📋 JIRA Ticket Update Process

After successfully creating a PR, you **MUST** update the associated JIRA ticket:

### Required Steps:
1. **Use atlassian-mcp-server** to access the JIRA issue
2. **Update Custom Field**: Set customfield_11170 ("Does this Work Include AI Assisted Code?") to "Yes"
3. **Use Correct Format**: 
   ```json
   {"customfield_11170": {"value": "Yes"}}
   ```
4. **Verify Success**: Confirm the field update was applied successfully
5. **Document**: Add a comment to the JIRA ticket referencing the created PR

### Critical Notes:
- This step is **mandatory** for all AI-assisted work
- Failure to update JIRA will result in incomplete task execution
- Always verify the field update before considering the task complete