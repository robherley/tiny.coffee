FROM node
WORKDIR /src

# Separate copy to cache node modules
COPY package*.json ./
RUN npm i

# Copy everything else over
COPY . .

# Run!
EXPOSE 3000
ENV COFFEE_PORT=3000
CMD ["npm", "start"]