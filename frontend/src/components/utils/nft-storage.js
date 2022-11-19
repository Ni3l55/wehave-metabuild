import { NFTStorage, File } from 'nft.storage'
import mime from 'mime'

const client = new NFTStorage({ token: 'eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiJkaWQ6ZXRocjoweGNFRDc3MEYxMjk1NDE4ODhEMWNGYWRGZUMwQzE5MzhhODBEMzJGODEiLCJpc3MiOiJuZnQtc3RvcmFnZSIsImlhdCI6MTY2NjQ0MzE0ODU4OSwibmFtZSI6IndlaGF2ZV90ZXN0bmV0In0.MAAF8Jz7waJS6mUczlIUS0nvvV82Q1q9XKHDb59vjQU' })

export async function saveOnIPFS(name, description, picture) {
  let metadata = new File(
    ['{title: ' + name + ', description: ' + description + '}'],
    'information.json',
    { type: 'text/json' }
  )

  let image = new File(
    [picture],
    'image.jpg',
    { type: 'image/jpg' }
  )

  const cid = await client.storeDirectory([metadata, image])
  console.log(cid);
  return cid;
}
