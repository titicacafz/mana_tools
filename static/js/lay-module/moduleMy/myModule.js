/**
 * ajax请求
 * @use 引入common.js  调用myModule.x方法调用对象
 * url, type, dataType, data, callback
 **/
layui.define(["jquery"], function (exports) {
  var $ = layui.jquery
  var myModule = {
    ajax: function (obj) {
      $.ajax({
        url: obj.url,
        type: obj.type,
        dataType: obj.dataType,
		contentType: 'application/json',
        data: obj.data,
        timeout: 120000, //超时时间设置，单位毫秒
        success: function (res) {
          switch (res.code) {
            case 0:
              if (obj.success) {
                obj.success(res)
              }
              break

            default:
              layer.alert(`请求失败${res.title}`)
              if (obj.error) {
                obj.error(res)
              }
              break
          }
        },
        error: function (res) {
          if (obj.error) {
            obj.error(res)
          }
        },
      })
    },
  }
  //输出接口
  exports("myModule", myModule)
})
