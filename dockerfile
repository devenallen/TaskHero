# Stage 1: Build the Rust application with Trunk
FROM rust:1.77 AS builder

# Install Trunk
RUN cargo install --locked trunk

# Set the working directory
WORKDIR /app

# Copy the project files into the container
COPY . .

# Build the project with Trunk, targeting WebAssembly
RUN trunk build --release

# Stage 2: Create a lightweight runtime environment
FROM nginx:alpine

# Copy the built WASM app from the builder stage to Nginx's default HTML folder
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose port 80 to serve the app
EXPOSE 80

# Start Nginx to serve the app
CMD ["nginx", "-g", "daemon off;"]
