# Use the official Rust image as the base
FROM rust:latest

# Set the working directory in the container
WORKDIR /app

# Copy the entire project into the container
COPY . .

# Build the project in release mode
RUN cargo build --release

# Expose the application port (if your app uses one)
# EXPOSE 8080  # Uncomment and change the port if needed

# Run the compiled application
CMD ["./target/release/task_hero"]
