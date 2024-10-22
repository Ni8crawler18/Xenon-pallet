import React, { useState } from 'react';
import { PlusCircle, Link2, Unlink, Key, Settings } from 'lucide-react';
import { Card, CardContent, CardHeader, CardTitle } from '@/components/ui/card';
import { Alert, AlertDescription } from '@/components/ui/alert';

const XenonDashboard = () => {
  const [activeChains, setActiveChains] = useState([
    { chainName: 'Ethereum', chainId: 1, address: '0x1234...5678' },
    { chainName: 'Polkadot', chainId: 0, address: '5GrwV...7dhw' }
  ]);

  const [publicKeys, setPublicKeys] = useState([
    { id: 'key1', type: 'Ed25519', fingerprint: 'ed25519-2023...' },
    { id: 'key2', type: 'Sr25519', fingerprint: 'sr25519-2023...' }
  ]);

  return (
    <div className="min-h-screen bg-gray-50 p-8">
      <div className="max-w-6xl mx-auto">
        <header className="mb-8">
          <h1 className="text-3xl font-bold text-gray-900 mb-2">Xenon DID Dashboard</h1>
          <p className="text-gray-600">Manage your decentralized identity across multiple chains</p>
        </header>

        <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
          {/* DID Status Card */}
          <Card className="bg-white">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Key className="h-5 w-5" />
                DID Status
              </CardTitle>
            </CardHeader>
            <CardContent>
              <Alert className="mb-4">
                <AlertDescription>
                  DID: did:xenon:5GrwV...7dhw
                </AlertDescription>
              </Alert>
              <div className="flex justify-end">
                <button className="px-4 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors">
                  Copy DID
                </button>
              </div>
            </CardContent>
          </Card>

          {/* Quick Actions Card */}
          <Card className="bg-white">
            <CardHeader>
              <CardTitle className="flex items-center gap-2">
                <Settings className="h-5 w-5" />
                Quick Actions
              </CardTitle>
            </CardHeader>
            <CardContent>
              <div className="grid grid-cols-2 gap-4">
                <button className="p-4 border rounded-lg hover:bg-gray-50 transition-colors flex flex-col items-center gap-2">
                  <Link2 className="h-6 w-6 text-blue-600" />
                  <span>Link Chain</span>
                </button>
                <button className="p-4 border rounded-lg hover:bg-gray-50 transition-colors flex flex-col items-center gap-2">
                  <PlusCircle className="h-6 w-6 text-green-600" />
                  <span>Add Key</span>
                </button>
              </div>
            </CardContent>
          </Card>

          {/* Linked Chains */}
          <Card className="md:col-span-2">
            <CardHeader>
              <CardTitle>Linked Chains</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {activeChains.map((chain) => (
                  <div key={chain.chainId} className="flex items-center justify-between p-4 border rounded-lg">
                    <div>
                      <h3 className="font-medium">{chain.chainName}</h3>
                      <p className="text-sm text-gray-600">{chain.address}</p>
                    </div>
                    <button className="p-2 text-red-600 hover:bg-red-50 rounded-full transition-colors">
                      <Unlink className="h-5 w-5" />
                    </button>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>

          {/* Public Keys */}
          <Card className="md:col-span-2">
            <CardHeader>
              <CardTitle>Public Keys</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="space-y-4">
                {publicKeys.map((key) => (
                  <div key={key.id} className="flex items-center justify-between p-4 border rounded-lg">
                    <div>
                      <h3 className="font-medium">{key.type}</h3>
                      <p className="text-sm text-gray-600">{key.fingerprint}</p>
                    </div>
                    <div className="flex gap-2">
                      <button className="px-3 py-1 text-sm border rounded-md hover:bg-gray-50">
                        Copy
                      </button>
                      <button className="px-3 py-1 text-sm border rounded-md text-red-600 hover:bg-red-50">
                        Remove
                      </button>
                    </div>
                  </div>
                ))}
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </div>
  );
};

export default XenonDashboard;
