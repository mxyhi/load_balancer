// server.js

const http = require("http");

const hostname = "127.0.0.1";
let port = 3000;

const worker = 10;

for (let i = 0; i < worker; i++) {
  http
    .createServer((req, res) => {
      res.statusCode = 200;
      res.setHeader("Content-Type", "text/plain");
      res.end(`server ${i}\n`);
      console.log(`server ${i}`, req.url);
    })
    .listen(port + i, hostname, (err) => {
      if (err) {
        console.log(err);
      }
      console.log(`Server running at http://${hostname}:${port + i}/`);
    });
}
