"use strict";

/** @type {import('sequelize-cli').Migration} */
module.exports = {
  async up(queryInterface, Sequelize) {
    await queryInterface.createTable("user", {
      id: {
        type: Sequelize.INTEGER,
        autoIncrement: true,
        primaryKey: true,
      },
      name: {
        type: Sequelize.STRING,
        unique: true,
        allowNull: false,
      },
      email: {
        type: Sequelize.STRING,
        unique: true,
        allowNull: false,
      },
      picture: Sequelize.STRING,
      hash: Sequelize.STRING,
      updatedAt: Sequelize.TIME,
      createdAt: Sequelize.TIME,
    });

    await queryInterface.createFunction(
      "search_user",
      [{ type: "text", name: "search" }],
      `SETOF "user"`,
      // `TABLE (name text, email text)`,
      "plpgsql",
      `RETURN query(SELECT * FROM "user" WHERE "name" ILIKE ('%' || search || '%') OR "email" ILIKE ('%' || search || '%'));`,
      ["immutable strict"]
    );
  },

  async down(queryInterface, Sequelize) {
    await queryInterface.dropFunction("search_user", [
      { type: "text", name: "search" },
    ]);
    await queryInterface.dropTable("user");
  },
};
