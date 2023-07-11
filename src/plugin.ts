import { makeExtendSchemaPlugin, gql } from "graphile-utils";
import { Build, Options, Plugin } from "postgraphile";
import fetch from "node-fetch";

const typeDefs = gql`
  type OtherServer {
    name: String
  }
  extend type Query {
    other_server: OtherServer
  }
`;

const resolvers = {
  Query: {
    other_server: async () => {
      return {};
    },
  },
  OtherServer: {
    name: async (query) => {
      return (await fetch("http://localhost:3333/json")).text();
    },
  },
};

const MyPlugin: Plugin = makeExtendSchemaPlugin(() => {
  return {
    typeDefs,
    resolvers,
  };
});

export { MyPlugin };
