import { makeExtendSchemaPlugin, gql } from "graphile-utils";
import { Build, Options, Plugin } from "postgraphile";

const typeDefs = gql`
  type MyType {
    a: String
  }
`;

const resolvers = {
  Query: {
    find_lalalala: async (_query, args, context, resolveInfo) => {
      console.log("DEBUGPRINT[5]: plugin.ts:12: _query=", _query);
      console.log("DEBUGPRINT[6]: plugin.ts:13: args=", args);
      console.log("DEBUGPRINT[7]: plugin.ts:14: context=", context);
      console.log("DEBUGPRINT[8]: plugin.ts:15: resolveInfo=", resolveInfo);
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
