animation:全部的动画效果
configs:自定义参数
lib:全局变量声明
gui:状态ui、游戏封面、场景切换和ui、暂停、受击
resources:图片、纹理集、音乐资源加载
world:游戏大厅ui及互动、游戏房间

//开发日志
1.  defaultplugins的set函数不能连续分多次写add_plugins，要放在一起写成.set(). set()的格式
2.  defaultplugins一定要放在app的最顶端，因为很多插件涉及defaultplugin的初始化，顺序错了会直接panic
3.  状态切换需要：use:bevy::bevy_dev_tools和.add_systems(Update, log_transitions::<GameState>)
4.  删除对象只需调用Query<Entity, With<Sprite>>即可包括所有对象，然后：
    for parent in &mut menu_items_query {
        commands.entity(parent).despawn_recursive();
    }即可删除对象
    但需要注意：如果此时有函数然在调用被删除的对象，就会造成panic导致程序崩溃。
    因此在定义函数和删除对象的时候注意通过状态切换来划分函数作用域
5. 音乐问题已解决，原因一是音乐组件在defaultplugin中，之前重复初始化到panic了；
    原因二是草丹的ogg文件损坏了！F！害的劳资花了一周半检查代码问题（0^0'''）
    还有就是bevy初始不支持wav文件，需要在toml中的feature注明才能使用(后续的tiff字体可能也有类似的问题，不过已经提前打开力（0v0）)

//Issue
1. 纹理集资源是否应该设置成一块缓存存放
2. 角色纹理集已设置成全局静态变量，考虑在character和animation中调整(to do)