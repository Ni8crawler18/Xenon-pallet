// src/services/xenonService.ts
import { ApiPromise, WsProvider } from '@polkadot/api';
import { web3Enable, web3AccountsSubscribe } from '@polkadot/extension-dapp';
import { InjectedAccountWithMeta } from '@polkadot/extension-inject/types';

export class XenonService {
  private api: ApiPromise | null = null;
  private accounts: InjectedAccountWithMeta[] = [];

  async connect(nodeUrl: string = 'ws://127.0.0.1:9944') {
    try {
      const provider = new WsProvider(nodeUrl);
      this.api = await ApiPromise.create({ provider });
      
      // Enable web3 extension
      const extensions = await web3Enable('Xenon DID Dashboard');
      if (extensions.length === 0) {
        throw new Error('No web3 extension found');
      }

      // Subscribe to accounts
      await web3AccountsSubscribe((accounts) => {
        this.accounts = accounts;
      });

      return true;
    } catch (error) {
      console.error('Connection error:', error);
      return false;
    }
  }

  async createDid(account: string) {
    if (!this.api) throw new Error('API not initialized');

    try {
      const tx = this.api.tx.xenon.createDid();
      const injector = await this.getInjector(account);
      
      return new Promise((resolve, reject) => {
        tx.signAndSend(account, { signer: injector.signer }, ({ status, events }) => {
          if (status.isInBlock) {
            events.forEach(({ event }) => {
              if (this.api?.events.xenon.DidDocumentCreated.is(event)) {
                resolve(event.data);
              }
            });
          }
        }).catch(reject);
      });
    } catch (error) {
      console.error('Create DID error:', error);
      throw error;
    }
  }

  async linkChain(account: string, chainName: string, chainId: number, address: string) {
    if (!this.api) throw new Error('API not initialized');

    try {
      const tx = this.api.tx.xenon.linkChain(
        stringToU8a(chainName),
        chainId,
        stringToU8a(address)
      );
      
      const injector = await this.getInjector(account);
      
      return new Promise((resolve, reject) => {
        tx.signAndSend(account, { signer: injector.signer }, ({ status, events }) => {
          if (status.isInBlock) {
            events.forEach(({ event }) => {
              if (this.api?.events.xenon.ChainLinked.is(event)) {
                resolve(event.data);
              }
            });
          }
        }).catch(reject);
      });
    } catch (error) {
      console.error('Link chain error:', error);
      throw error;
    }
  }

  async unlinkChain(account: string, chainId: number) {
    if (!this.api) throw new Error('API not initialized');

    try {
      const tx = this.api.tx.xenon.unlinkChain(chainId);
      const injector = await this.getInjector(account);
      
      return new Promise((resolve, reject) => {
        tx.signAndSend(account, { signer: injector.signer }, ({ status, events }) => {
          if (status.isInBlock) {
            events.forEach(({ event }) => {
              if (this.api?.events.xenon.ChainUnlinked.is(event)) {
                resolve(event.data);
              }
            });
          }
        }).catch(reject);
      });
    } catch (error) {
      console.error('Unlink chain error:', error);
      throw error;
    }
  }

  async getDidDocument(account: string) {
    if (!this.api) throw new Error('API not initialized');

    try {
      const document = await this.api.query.xenon.didDocuments(account);
      return document.unwrapOr(null);
    } catch (error) {
      console.error('Get DID document error:', error);
      throw error;
    }
  }

  private async getInjector(account: string) {
    const accountData = this.accounts.find(acc => acc.address === account);
    if (!accountData) throw new Error('Account not found');
    return accountData.meta.source;
  }
}

// Helper function to convert string to Uint8Array
function stringToU8a(str: string): Uint8Array {
  return new TextEncoder().encode(str);
}

export const xenonService = new XenonService();
