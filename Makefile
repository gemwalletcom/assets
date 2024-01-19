#! /usr/bin/make -f

clean:
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