<script lang="ts">
    import { invoke } from "@tauri-apps/api";

    async function encrypt(to_encrypt: string, key:string): Promise<string> {
      let encrypted: string = await invoke('encrypt_string', { toEncrypt: to_encrypt, key: key })
      return encrypted
    }

    async function decrypt(to_decrypt: string, key:string): Promise<string> {
      let decrypted: string = await invoke('decrypt_cypher', { toDecrypt: to_decrypt, key: key })
      return decrypted
    }


</script>
{#await encrypt("Heelooooooooo", "test") then value }
    {value}    
{/await}
{#await decrypt("DAJVpJUlJ0TIHF82BEW/zA==", "test") then value }
    {value}    
{/await}
