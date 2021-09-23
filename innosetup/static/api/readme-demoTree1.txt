

树形图数据返回格式：
{
	"status": {
		"code": 200, //接口请求成功标志
		"message": "操作成功"
	},
	"data": [{
			"title": "湖南省", //节点名称
			"basicData": {"full_path": "该节点的额完整路径"}, //basicData为一个对象，full_path字段存放改节点的完整路径 用于回传给下载接口
			"last": false, //是否无子节点（若该节点为文件夹，则"last": false    若该节点已经是日志文件则 "last": true）
			"children": [] //字节点（目前为异步加载树结构 所以每次返回children为空数组即可）
		},
		{

			"title": "湖北省",
			"basicData": {"full_path": "自定义数据1", "data2": "自定义数据2", "data3": "自定义'我带了单引号'3"},
			"last": true,
			"children": []
		},
		{
			"title": "广东省",
			"basicData": {"full_path": "自定义数据1", "data2": "自定义数据2", "data3": "自定义'我带了单引号'3"},
			"last": false,
			"children": []
		},
		{
			"title": "浙江省",
			"basicData": {"full_path": "自定义数据1", "data2": "自定义数据2", "data3": "自定义'我带了单引号'3"},
			"last": false,
			"children": []
		},
		{
			"title": "福建省",
			"basicData": {"full_path": "自定义数据1", "data2": "自定义数据2", "data3": "自定义'我带了单引号'3"},
			"last": false,
			"children": []
		}
	]

}