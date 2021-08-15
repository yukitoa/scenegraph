use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::default::Default;
use uuid::Uuid;

//SceneGraphのノード
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Node {
    instanceid: Uuid,
    parentid: Option<Uuid>,
    childs: Vec<Uuid>,
}
//SceneGraph
#[derive(Serialize, Deserialize, Debug)]
pub struct SceneGraph {
    nodes: HashMap<Uuid, Node>,
    root: Uuid,
}
impl Default for SceneGraph {
    fn default() -> SceneGraph {
        let root_uuid = SceneGraph::generateuuid();
        let mut scene_graph = SceneGraph {
            nodes: HashMap::<Uuid, Node>::new(),
            root: root_uuid,
        };
        scene_graph.nodes.insert(
            root_uuid,
            Node {
                instanceid: root_uuid,
                parentid: None,
                childs: Vec::<Uuid>::new(),
            },
        );
        scene_graph
    }
}
impl SceneGraph {
    fn generateuuid() -> Uuid {
        Uuid::new_v4()
    }
    //nodeを得る
    pub fn get(&self, instance_id: &Uuid) -> Option<&Node> {
        return self.nodes.get(instance_id);
    }
    //nodeを得る(内部使用)
    fn get_mut(&mut self, instance_id: &Uuid) -> Option<&mut Node> {
        return self.nodes.get_mut(instance_id);
    }
    // root を得る
    pub fn get_root(&self) -> Option<&Node> {
        return self.nodes.get(&self.root);
    }
    //RootのIDを得る
    pub fn get_root_id(&self) -> Uuid {
        return self.root;
    }
    //Nodeを作る
    pub fn create_node(&mut self) -> Uuid {
        let uuid = SceneGraph::generateuuid();
        self.nodes.insert(
            uuid,
            Node {
                instanceid: uuid,
                parentid: None,
                childs: Vec::<Uuid>::new(),
            },
        );
        uuid
    }
    //Nodeを削除する
    pub fn delete_node_call<F>(&mut self, this_id: &Uuid, on_delete_func: &F)
    where
        F: Fn(&Uuid) -> (),
    {
        (*on_delete_func)(this_id);
        self.remove_parent(this_id);
        self.nodes.remove(this_id);
    }
    //Nodeを削除する(再帰的にchildNodeも削除する)
    pub fn delete_node_recursive_call<F>(&mut self, this_id: &Uuid, on_delete_func: &F)
    where
        F: Fn(&Uuid) -> (),
    {
        if let Some(instnace) = self.get(this_id) {
            let _childs = instnace.childs.clone();
            for id in _childs {
                self.delete_node_recursive_call(&id, on_delete_func);
            }
            self.delete_node_call(this_id, on_delete_func);
        }
    }
    //Nodeを削除する
    pub fn delete_node(&mut self, this_id: &Uuid) {
        self.remove_parent(this_id);
        self.nodes.remove(this_id);
    }
    //Nodeを削除する(再帰的にchildNodeも削除する)
    pub fn delete_node_recursive(&mut self, this_id: &Uuid) {
        if let Some(instnace) = self.get(this_id) {
            let _childs = instnace.childs.clone();
            for id in _childs {
                self.delete_node_recursive(&id);
            }
            self.delete_node(this_id);
        }
    }
    //親のIDを得る
    pub fn get_parent_id(&self, this_id: &Uuid) -> Option<Uuid> {
        if let Some(instance) = self.get(this_id) {
            instance.parentid
        } else {
            None
        }
    }
    //子供のIDを得る
    pub fn get_childs(&self, this_id: &Uuid) -> Option<&Vec<Uuid>> {
        if let Some(instance) = self.get(this_id) {
            Some(&instance.childs)
        } else {
            None
        }
    }
    //子供のIDを得る
    fn get_mut_childs(&mut self, this_id: &Uuid) -> Option<&mut Vec<Uuid>> {
        if let Some(instance) = self.get_mut(this_id) {
            Some(&mut instance.childs)
        } else {
            None
        }
    }
    //兄弟(自分も含む)を得る
    pub fn get_siblings(&self, this_id: &Uuid) -> Option<&Vec<Uuid>> {
        if let Some(parent_id) = self.get_parent_id(this_id) {
            return self.get_childs(&parent_id);
        }
        None
    }
    //自分の兄弟間の順番を得る
    pub fn get_sibling_index(&self, this_id: &Uuid) -> Option<usize> {
        if let Some(siblings) = self.get_siblings(this_id) {
            if let Some(pair) = siblings
                .iter()
                .enumerate()
                .find(|&item| *item.1 == *this_id)
            {
                return Some(pair.0);
            }
        }
        return None;
    }
    //自分の兄弟間の順番を得る
    pub fn get_mut_sibling_index(&mut self, this_id: &Uuid) -> Option<usize> {
        if let Some(siblings) = self.get_siblings(this_id) {
            if let Some(pair) = siblings
                .iter()
                .enumerate()
                .find(|&item| *item.1 == *this_id)
            {
                return Some(pair.0);
            }
        }
        return None;
    }
    //兄弟(自分も含む)を得る
    fn get_mut_siblings(&mut self, this_id: &Uuid) -> Option<&mut Vec<Uuid>> {
        if let Some(parent_id) = self.get_parent_id(this_id) {
            return self.get_mut_childs(&parent_id);
        }
        None
    }
    //自分の兄弟間の順番を設定する
    fn _set_sibling_index(&mut self, index: usize, this_id: &Uuid) {
        if let Some(siblings) = self.get_mut_siblings(this_id) {
            siblings.insert(index, this_id.clone());
        }
    }
    //自分の兄弟間の順番を設定する
    fn _remove_sibling_index(&mut self, this_id: &Uuid) {
        if let Some(siblings) = self.get_mut_siblings(this_id) {
            siblings.retain(|&item| item != *this_id);
        }
    }

    //自分の兄弟間の順番を変更する
    pub fn change_sibling_index(&mut self, index: usize, this_id: &Uuid) {
        self._remove_sibling_index(this_id);
        self._set_sibling_index(index, this_id);
    }

    //親から外す(deleteはしない。迷子になるのでどこかに繋ぐかdelete_nodeすること)
    pub fn remove_parent(&mut self, this_id: &Uuid) {
        //parentからthisを省く
        if let Some(parent_id) = self.get_parent_id(this_id) {
            if let Some(parent_instance) = self.nodes.get_mut(&parent_id) {
                parent_instance.childs.retain(|&item| item != *this_id);
            }
        }
        //thisからparentを省く
        if let Some(instance) = self.nodes.get_mut(this_id) {
            instance.parentid = None;
        }
    }
    pub fn child_len(&self, this_id: &Uuid) -> usize {
        if let Some(instance) = self.nodes.get(this_id) {
            instance.childs.len()
        } else {
            0
        }
    }
    //親を設定する
    pub fn set_parent(&mut self, this_id: &Uuid, parent_id: &Uuid) {
        self.remove_parent(this_id);
        //thisにparentを設定する
        if let Some(instance) = self.nodes.get_mut(this_id) {
            instance.parentid = Some(parent_id.clone());
        }
        //parentにthisを登録する
        if let Some(parent_instance) = self.nodes.get_mut(&parent_id) {
            parent_instance
                .childs
                .insert(parent_instance.childs.len(), this_id.clone());
        }
    }
    //rootに追加する
    pub fn set_parent_root(&mut self, this_id: &Uuid) {
        self.set_parent(this_id, &self.root.clone());
    }
}
#[test]
fn scene_graph_test() {
    let mut scenegraph = SceneGraph::default();
    let id_1 = scenegraph.create_node();
    scenegraph.set_parent_root(&id_1);
    let id_2 = scenegraph.create_node();
    scenegraph.set_parent(&id_2, &id_1);
    let id_30 = scenegraph.create_node();
    scenegraph.set_parent(&id_30, &id_2);
    let id_31 = scenegraph.create_node();
    scenegraph.set_parent(&id_31, &id_2);
    assert_eq!(1, scenegraph.get_sibling_index(&id_31).unwrap());
    scenegraph.change_sibling_index(0, &id_31);
    assert_eq!(0, scenegraph.get_sibling_index(&id_31).unwrap());
    assert_eq!(2, scenegraph.get_childs(&id_2).unwrap().len());
    {
        let node_2 = scenegraph.get(&id_2).unwrap();
        assert_eq!(node_2.childs.len(), 2);
    }
    {
        if let Some(childs_list) = scenegraph.get_childs(&id_1) {
            for id in childs_list {
                assert_eq!(*id, id_2);
            }
        }
    }
    scenegraph.delete_node_recursive_call(&id_30, &|_id: &Uuid| {});
    {
        let node_2 = scenegraph.get(&id_2).unwrap();
        assert_eq!(node_2.childs.len(), 1);
    }
    scenegraph.delete_node_recursive(&id_2);
    use serde_json::to_string;
    let json_enc = to_string(&scenegraph).ok();
    println!("{}", json_enc.unwrap());
    assert_eq!(scenegraph.nodes.len(), 2); //rootがある
}
