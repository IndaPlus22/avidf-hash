pub mod hash;
pub mod data;

use anyhow::{Context, Result, bail};
use std::fmt::Debug;
use data::Data;

use self::hash::Hashable;



#[derive(Debug)]
pub struct HashTable<K, V> {
    pub capacity: u32,
    load_factor: f32,
    size: u32,
    pub table: Vec<Option<Vec<Data<K, V>>>>,
}

impl <K, V> HashTable<K, V> 
where
    K: Hashable + Clone + Debug + Eq,
    V: Clone + Debug
{
    pub fn new(capacity: u32) -> Self {
        let mut table = Vec::with_capacity(capacity as usize);
        for _ in 0..capacity {
            table.push(None);
        }
        
        HashTable {
            capacity: capacity,
            load_factor: 0.75,
            size: 0,
            table: table,
        }
    }

    //Insert function
    pub fn insert(&mut self, data: Data<K, V>) -> Result<()> {
        let index = self.compress(data.key.hash());

        match &mut self.table[index] {
            Some(_vec) => {
                _vec.push(data);
                self.size += 1;
            },
            None => {
                self.table[index] = Some(vec![data]);
                self.size += 1;
            }
        }

        if (self.size as f32 / self.capacity as f32 ) >= self.load_factor { 
            self.resize()
                .with_context(|| format!("Failed to resize table"))?
        }

        Ok(())
    }


    //Delete function
    pub fn delete(&mut self, key: K) -> Result<()> {
        let index = self.compress(key.hash());

        match &mut self.table[index] {
            Some(_vec) => {
                if _vec.len() == 1 {
                    if key == _vec[0].key {
                        self.table[index] = None;
                        Ok(())
                    } else {
                        bail!("Failed to find the used  key")
                    }
                } else {
                    let mut key_found = false;
                    for i in 0.._vec.len() {
                        if key == _vec[i].key {
                            _vec.swap_remove(i);
                            key_found = true;
                        }
                    }
                    if key_found {
                        Ok(())
                    } else {
                        bail!("Failed to find the used  key")
                    }
                }
            },
            None => bail!("Failed to find the used  key")
        }
    }


    //get function
    pub fn get(&mut self, key: K) -> Result<V> {
        let index = self.compress(key.hash());
        let mut value: Option<V> = None;

        match &mut self.table[index] {
            Some(_vec) => {
                for i in 0.._vec.len() {
                    if key == _vec[i].key {
                      value = Some(_vec[i].value.clone());
                    }
                }
            },
            None => ()
        }
        match value {
            Some(_value) => Ok(_value),
            None => bail!("Failed to find the used key"),
        }
    }


    //Print function 
    pub fn print(&self) -> Result<()> {
        let mut output = format!("\n========== Table ==========\n");
        for _bucket in &self.table {
            match _bucket { 
                Some(_vec) => {
                    let key = &_vec[0].key;
                    output = format!("{}Key: {:?}, ", output, key);
                    for _data in _vec {
                        let value = &_data.value;
                        output = format!("{}Value: {:#?}, ", output, value);
                    } 
                    output = format!("{}\n", output);
                },
                None =>  output = format!("{}-----\n", output),
            }
        }
        println!("{}", output);
        Ok(())
    }


    //Resize the table
    fn resize(&mut self) -> Result<()> {
        self.capacity *= 2;
        let mut table = HashTable::<K, V>::new(self.capacity);
        for _bucket in &self.table {
            match _bucket { 
                Some(_vec) => {
                    for _data in _vec {
                        let data = _data.clone();
                        table.insert(data).with_context(|| format!("Failed to insert data into new table"))?;
                    }
                },
                None => continue
            }
        }
        *self = table;
        Ok(())
    }

    fn compress(&self, hash: u32) -> usize {
        (hash % self.capacity) as usize
    }
} 