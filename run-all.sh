#!/bin/bash

# Step 1: Build Database Docker Image and Start Replicas
docker-compose -f docker-compose-db.yml up -d

# Step 2: Build Redis Docker Image and Start Redis
docker-compose -f docker-compose-redis.yml up -d

# Step 3: Build API Server Docker Image
# docker-compose build

# # Step 4: Start API Server
# docker-compose -f docker-compose.yml up
