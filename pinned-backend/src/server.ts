import Fastify from "fastify";
import dotenv from "dotenv";
import path from "path";

dotenv.config({
  "path": path.join(__dirname + "../.env")
});

const pinned_server = Fastify({
  "logger": true
});

pinned_server.listen({port: 3001}, (err, address) => {
  if (err) {
    pinned_server.log.error(err);
    process.exit(1);
  }
  pinned_server.log.info(`Listeneing on ${address}`);
});
