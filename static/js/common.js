window.rootPath = (function (src) {
  src = document.scripts[document.scripts.length - 1].src
  return src.substring(0, src.lastIndexOf("/") + 1)
})()
layui
  .config({
    base: `${window.rootPath}moduleMy`, //自定义layui组件的目录
  })
  .extend({
    //设定组件别名
    myModule: `{/}${window.rootPath}lay-module/moduleMy/myModule`,
  })
