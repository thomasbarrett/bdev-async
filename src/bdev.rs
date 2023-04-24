use async_trait::async_trait;

pub trait BlockDevice {
    fn logical_block_size(&self) -> usize;
    fn size(&self) -> usize;
}

#[async_trait(?Send)]
pub trait BlockDeviceQueue {
    fn logical_block_size(&self) -> usize;
    fn size(&self) -> usize;
    async fn read_at(&self, buf: &mut [u8], offset: u64) -> std::io::Result<usize>;
    async fn write_at(&self, buf: &[u8], offset: u64) -> std::io::Result<usize>;
}
