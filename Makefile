#! /usr/bin/make -f

REMOVE_CHAIN = binance tomochain nimiq ontology thundertoken zilliqa viacoin verge wanchain waves iotex icon harmony gochain groestlcoin goerlitestnet aeternity arbitrumgoerli avalanchecfuji band bitcoincash bitcoingold btcdiamond callisto classic digibyte ellaism energyweb eos ether-1 ethereumpow everscale iost iotexevm kusama komodo qtum steem syscoin tftm polygonmumbai poa optimismgoerli nuls oasis neutrontestnet moonbasealpha aion aryacoin

clean: clean_chains clean_abandoned_assets
	@rm -rf -f cmd
	@rm -rf -f internal
	@rm -rf -f dapps
	@rm -f -- .golangci.yml go.mod 
	@rm -f -- go.mod 
	@rm -f -- go.sum

	@rm -rf -f blockchains/solana/validators/assets
	@rm -rf -f blockchains/cosmos/validators/assets
	@rm -rf -f blockchains/osmosis/validators/assets
	@rm -rf -f blockchains/tia/validators/assets
	@rm -rf -f blockchains/nativeinjective/validators/assets
	@rm -rf -f blockchains/sei/validators/assets

clean_chains:
	@for chain in $(REMOVE_CHAIN); do \
		rm -rf -f blockchains/$$chain; \
	done

clean_abandoned_assets:
	@find . -name "info.json" -exec grep -q "abandoned" {} \; -exec dirname {} \; | sort -u | xargs rm -rf