from collections import defaultdict


def make():
    res = [] # 存储最终结果
    data = [
        {'id': 1, 'parent_id': 2, 'name': "Node1"},
        {'id': 2, 'parent_id': 5, 'name': "Node2"},
        {'id': 3, 'parent_id': 0, 'name': "Node3"},
        {'id': 4, 'parent_id': 5, 'name': "Node4"},
        {'id': 5, 'parent_id': 0, 'name': "Node5"},
        {'id': 6, 'parent_id': 3, 'name': "Node6"},
        {'id': 7, 'parent_id': 3, 'name': "Node7"},
        {'id': 8, 'parent_id': 0, 'name': "Node8"},
        {'id': 9, 'parent_id': 1, 'name': "Node9"}
    ]
    # tmp 存储每个数据的ID 与 数据 的映射 例如：
    # {
    #     1: {'id': 1, 'parent_id': 2, 'name': "Node1"},
    #     2: {'id': 2, 'parent_id': 5, 'name': "Node2"}
    # }
    tmp = {}
    pmap = defaultdict(list)  # 存储每个父节点 挂载的所有 子节点
    for item in data:
        tmp[item['id']] = item
        pmap[item['parent_id']].append(item['id'])

    def dfs(item):
        """
        dfs含义是 返回该数据所对应的节点
        """
        t = {
            'id': item['id'],
            'name': item['name'],
            'children': [],
        }
        # 遍历该节点的所有子节点 如果有子节点就放到 该节点的children里
        for zid in pmap[item['id']]:
            t['children'].append(dfs(tmp[zid]))
        return t
    # 从根节点的子节点出发遍历就行了
    for id in pmap[0]:
        res.append(dfs(tmp[id]))
    print(res)

make()
