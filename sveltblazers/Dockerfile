FROM node:20

WORKDIR /app

COPY package*.json ./

COPY svelte.config.js ./

COPY vite.config.ts ./

RUN npm install

COPY ./src ./src
COPY ./static ./static

RUN npm run build

EXPOSE 5173

CMD ["npm", "run", "dev"]
