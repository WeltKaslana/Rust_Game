animation:全部的动画效果
configs:自定义参数
lib:全局变量声明
gui:状态ui、游戏封面、场景切换和ui、暂停、受击
resources:图片、纹理集、音乐资源加载
world:游戏大厅ui及互动、游戏房间

//开发日志
1.defaultplugins的set函数不能连续分多次写add_plugins，要放在一起写成.set().set()的格式
2.defaultplugins一定要放在app的最顶端，因为很多插件涉及defaultplugin的初始化，顺序错了会直接panic
3.状态切换需要：use:bevy::bevy_dev_tools和.add_systems(Update, log_transitions::<GameState>)