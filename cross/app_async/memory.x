MEMORY
{
  /* NOTE 1 K = 1 KiBi = 1024 bytes */
  /* These values correspond to the NRF52833 with Softdevices S140 7.3.0 */
  /* FLASH : ORIGIN = 0x00000000 + 156K, LENGTH = 512K - 156K */
  /*  RAM : ORIGIN = 0x20000000 + 62K, LENGTH = 128K - 62K    */
    FLASH : ORIGIN = 0x00000000, LENGTH = 512K
    RAM : ORIGIN = 0x20000000, LENGTH = 128K
}

