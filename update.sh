#!/bin/bash

# List of images to ignore
IGNORE_IMAGES=("postgres" "mongodb")

# Function to check if an image should be ignored
should_ignore() {
  local image=$1
  for ignore in "${IGNORE_IMAGES[@]}"; do
    if [[ $image == *$ignore* ]]; then
      return 0  # true, should ignore
    fi
  done
  return 1  # false, should not ignore
}

services=$(docker-compose config --services)

for service in $services; do
  image=$(docker-compose config | grep "$service:" -A 2 | grep 'image:' | awk '{print $2}')
  
  if [ -n "$image" ]; then
    # Check if the image should be ignored
    if should_ignore "$image"; then
      echo "Skipping check for image: $image"
      continue
    fi
    
    echo "Checking for updates for image: $image"
    
    # Get local and remote digests
    local_digest=$(docker image inspect $image --format '{{index .RepoDigests 0}}' | cut -d'@' -f2)
    remote_digest=$(docker pull --quiet --disable-content-trust $image > /dev/null 2>&1 && docker image inspect $image --format '{{index .RepoDigests 0}}' | cut -d'@' -f2)

    if [ "$local_digest" != "$remote_digest" ]; then
      docker-compose pull
      docker-compose up -d --force-recreate
      docker image prune -f
      docker pull ghcr.io/mathisburger/cc-images-java:latest
      docker pull golang:1.19
      cd web && docker-compose pull && docker-compose up -d --force-recreate
      exit 0
    else
      echo "$image is up to date."
    fi
  fi
done

echo "No new images found."
