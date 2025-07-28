# Learning

Welcome to the Tansu project! This document provides a high-level overview of each module and describes the purpose of key files within them to help new contributors understand the architecture and code layout. File references have been updated to match the actual codebase structure.

---

## Modules and Key Files

### 1. **tansu-broker**
- **src/broker.rs**: Main broker logic, implements Kafka protocol handlers and request routing.
- **src/lib.rs**: Library entry point for the broker crate.
- **src/config.rs**: Configuration parsing and management for the broker.
- **tests/**: Integration tests for broker functionality.

### 2. **tansu-sans-io**
- **src/record/codec.rs**: Serialization and deserialization of Kafka records.
- **src/message/**: Definitions for Kafka protocol messages.
- **src/error.rs**: Error types for protocol handling.
- **src/lib.rs**: Library entry point for protocol logic.
- **tests/**: Protocol compatibility and wire format tests.

### 3. **tansu-storage**
- **src/pg.rs**: PostgreSQL storage backend implementation.
- **src/s3.rs**: S3/Minio storage backend implementation.
- **src/memory.rs**: In-memory storage backend for testing and development.
- **src/lib.rs**: Storage engine trait and abstraction.
- **tests/**: Storage engine tests.

### 4. **tansu-schema**
- **src/avro.rs, src/json.rs, src/protobuf.rs**: Schema validation for Avro, JSON, and Protobuf.
- **src/registry.rs**: Schema registry logic for managing topic schemas.
- **src/lake/berg.rs**: Iceberg data lake integration and table management.
- **src/lib.rs**: Library entry point for schema logic.
- **tests/**: Schema validation and registry tests.

### 5. **tansu-cli**
- **src/cli.rs**: Main CLI entrypoint and argument parsing.
- **src/commands/**: Subcommands for topic management, producing, consuming, etc.
- **src/lib.rs**: Library entry point for CLI logic.

### 6. **tansu-cat**
- **src/produce.rs**: Logic for producing messages to topics.
- **src/consume.rs**: Logic for consuming messages from topics.
- **src/validate.rs**: Schema validation for produced/consumed messages.
- **src/lib.rs**: Library entry point for cat tool logic.

### 7. **tansu-generator**
- **src/lib.rs**: Data generation logic for topics.
- **src/schema.rs**: Schema-based fake data generation.

### 8. **tansu-topic**
- **src/admin.rs**: Topic administration logic (create, delete, list).
- **src/lib.rs**: Library entry point for topic tool logic.

### 9. **tansu-proxy**
- **src/proxy.rs**: Kafka API proxy logic.
- **src/lib.rs**: Library entry point for proxy logic.

### 10. **tansu-service**
- **src/service.rs**: API layers, request routing, and service abstractions.
- **src/context.rs**: Context management for service requests.
- **src/error.rs**: Service error types.
- **src/lib.rs**: Library entry point for service logic.

### 11. **tansu-model**
- **src/message.rs**: Core message types used throughout the codebase.
- **src/protocol.rs**: Protocol model definitions.
- **src/lib.rs**: Library entry point for model logic.

### 12. **tansu-otel**
- **src/lib.rs**: OpenTelemetry integration for tracing and metrics.
- **src/config.rs**: Observability configuration.

---

## How to Use This Document

- Start by exploring the main files in each module to understand their responsibilities.
- Use the CLI tools to interact with the broker and storage backends.
- Review the schema and storage modules to see how data is validated and persisted.
- Check the service and model modules for request handling and data types.
- Enable observability with the otel module for tracing and metrics.

---

## Additional Resources

- See the `README.md` for setup instructions and more details.
- Refer to `CONTRIBUTIONS.md` for guidelines on contributing.
- Use the CLI tools and Docker Compose services to experiment with different configurations.

---

Happy