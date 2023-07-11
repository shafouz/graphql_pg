"use strict";

/** @type {import('sequelize-cli').Migration} */
module.exports = {
  async up(queryInterface, Sequelize) {
    await queryInterface.createTable("domain", {
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
      tags: {
        type: Sequelize.JSONB,
        defaultValue: "{}",
      },
      updatedAt: Sequelize.TIME,
      createdAt: Sequelize.TIME,
    });

    await queryInterface.createFunction(
      '"searchTagByValue"',
      [{ type: "text", name: "search" }],
      `SETOF "domain"`,
      "plpgsql",
      `RETURN query(SELECT * FROM domain WHERE tags->>search IS NOT NULL);`,
      ["immutable strict"]
    );
  },

  async down(queryInterface, Sequelize) {
    await queryInterface.dropFunction("searchTagByValue", []);
    await queryInterface.dropTable("domain");
  },
};
