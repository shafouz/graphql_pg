"use strict";

/** @type {import('sequelize-cli').Migration} */
module.exports = {
  async up(queryInterface, Sequelize) {
    await queryInterface.bulkInsert(
      "domain",
      [
        {
          name: "laalsdlasld.com",
          tags: JSON.stringify({ something: "something" }),
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        {
          name: "asdasdasd.com",
          tags: JSON.stringify({ something: "asdasdas" }),
          createdAt: new Date(),
          updatedAt: new Date(),
        },
      ],
      {}
    );
  },

  async down(queryInterface, Sequelize) { },
};
