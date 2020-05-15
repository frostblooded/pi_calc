docker build -t pi-calc . --no-cache
docker tag pi-calc frostblooded/pi-calc
docker push frostblooded/pi-calc
