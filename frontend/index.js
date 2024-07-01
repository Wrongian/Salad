import { handler } from "./build/handler.js";
import "dotenv/config";
import express from "express";

const FRONTEND_SERVER_PORT = process.env.FRONTEND_SERVER_PORT;

const app = express();
console.dir(handler);

app.use(handler);

app.listen(FRONTEND_SERVER_PORT, () => {
  console.log(`listening on port ${FRONTEND_SERVER_PORT}`);
});
