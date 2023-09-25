# Define the service name and image name
echo "ensure you've created a builder `docker buildx create --use`"
SERVICE_NAME="portfolio_api"
NEW_IMAGE_NAME="portfolio_api:latest"

# Build the updated Docker image using Buildx
sudo docker buildx build -t $NEW_IMAGE_NAME -f ./Dockerfile.api . --load

# Check if the service already exists
if [[ "$(sudo docker compose ps -q $SERVICE_NAME)" ]]; then
  # If the service exists, stop and remove it
  sudo docker-compose stop $SERVICE_NAME
  sudo docker-compose rm -f $SERVICE_NAME
fi

# Update the Docker Compose file to use the new image
sed -i "s/image: $SERVICE_NAME:.*/image: $NEW_IMAGE_NAME/" docker-compose.yml

# Redeploy the updated service
sudo docker-compose up -d $SERVICE_NAME

echo "Service $SERVICE_NAME updated and redeployed with the new image."
