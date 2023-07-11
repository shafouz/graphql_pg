import { createServer } from "http";
import { postgraphile } from "postgraphile";
import { MyPlugin } from "./src/plugin.js";

const port = process.env.PORT || 3000;
console.log(`Server started: ${port}`);

const server = createServer(
  postgraphile(
    "postgres://postgres:postgres@127.0.0.1:5432/postgres",
    "public",
    {
      watchPg: true,
      graphiql: true,
      enhanceGraphiql: true,
      exportGqlSchemaPath: "schema.graphql",
      // appendPlugins: [MyPlugin],
    }
  )
);

server.listen(port);
