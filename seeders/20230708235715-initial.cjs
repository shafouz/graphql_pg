"use strict";

/** @type {import('sequelize-cli').Migration} */
module.exports = {
  async up(queryInterface, Sequelize) {
    await queryInterface.bulkInsert(
      "user",
      [
        {
          name: "laalsdlasld",
          email: "laalsdlasld",
          createdAt: new Date(),
          updatedAt: new Date(),
        },
        {
          name: "asdasdasd",
          email: "asdasdasd",
          createdAt: new Date(),
          updatedAt: new Date(),
        },
      ],
      {}
    );
  },

  async down(queryInterface, Sequelize) {},
};
