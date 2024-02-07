#! /usr/bin/make -f

REMOVE_CHAIN = zetaevm fio fusion bnbt acalaevm bluzelle celo cfxevm evmos cronos hoo kavaevm metis metter scroll ronin sepolia platon neblio nebulas moonbeam neon ean stratis cardano sommelier axelar tezos near cryptoorg mars juno comdex stargaze stride nativeevmos agoric fetch coreum crescent teritori kava loom nativecanto secret akash tia acala firo heco iris kcc kujira moonriver nano neo persistence quasar umee xdai zen xdc zelcash smartbch elrond decred ecash hedera kin ravencoin pivx terra terrav2 vechain ark theta binance tomochain nimiq ontology thundertoken zilliqa viacoin verge wanchain waves iotex icon harmony gochain groestlcoin goerlitestnet aeternity arbitrumgoerli avalanchecfuji band bitcoincash bitcoingold btcdiamond callisto classic digibyte ellaism energyweb eos ether-1 ethereumpow everscale iost iotexevm kusama komodo qtum steem syscoin tftm polygonmumbai poa optimismgoerli nuls oasis neutrontestnet moonbasealpha aion aryacoin
FIX_VALIDATORS = solana cosmos osmosis tia sei nativeinjective sei tron sui tezos polkadot

pull:
	git merge upstream/master

clean: clean_chains clean_abandoned_assets
	@rm -rf -f cmd
	@rm -rf -f internal
	@rm -rf -f dapps
	@rm -f -- .golangci.yml go.mod 
	@rm -f -- go.mod 
	@rm -f -- go.sum

clean_chains:
	@for chain in $(REMOVE_CHAIN); do \
		rm -rf -f blockchains/$$chain; \
	done
	@for chain in $(FIX_VALIDATORS); do \
		rm -rf -f blockchains/$$chain/validators/assets; \
	done

clean_abandoned_assets:
	@find . -name "info.json" -exec grep -q "abandoned" {} \; -exec dirname {} \; | sort -u | xargs rm -rf