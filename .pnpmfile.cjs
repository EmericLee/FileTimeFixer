module.exports = {
  hooks: {
    readPackage(pkg, context) {
      // 确保所有依赖都正确安装
      return pkg;
    },
  },
};