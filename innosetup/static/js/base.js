//接口请求地址及端口
let ajaxUrlSet = ""
//动态计算页面高度
function getBaseHeight() {
  let basicHeight = $(window).height() - 50
  return basicHeight
}
//文件流下载
function downLoadFile(url, data, method, isNewWinOpen) {
  var config = {
    url: url,
    data: (data = data || {}),
    method: (method = method || "GET"),
    isNewWinOpen: (isNewWinOpen = isNewWinOpen || false),
  }
  var $iframe = $('<div style="display: none"><iframe id="down-file-iframe" name="down-file-iframe" /></div>')
  var $form = $('<form target="down-file-iframe" method="' + config.method + '" action="' + config.url + '" />')
  if (config.isNewWinOpen) {
    $form.attr("target", "_blank")
  }
  /*拼接参数*/
  for (var key in config.data) {
    $form.append('<input type="hidden" name="' + key + '" value="' + config.data[key] + '" />')
  }
  $iframe.append($form)
  $(document.body).append($iframe)
  console.log("11111")
  $form.submit()
  setTimeout(function () {
    $iframe.remove()
  }, 1000)
}
