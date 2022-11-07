#!/bin/sh
SSL_DIR=/cockroach/ssl
CERT_DIR=${SSL_DIR}/certs
KEY_DIR=${SSL_DIR}/keys

set -e

if [ ! -f "/.dockerenv" ]; then
	printf "%s\n" "This script is meant as init-script for cockroachdb in docker. Refusing to run here."
	exit 0
fi

# Do we have expected volume/mounts? Create dir if nothing's there.
for d in "${SSL_DIR}" "${CERT_DIR}" "${KEY_DIR}"
do
	if [ ! -d "${d}" ]; then
		if [ ! -e "${d}" ]; then
			printf "%s\n"  "Creating non-existing dir ${d}"
			mkdir -p "${d}"
		else
			printf "%s\n" "SSL related directory expected at ${d} is not a dir. Refusing."
			exit 1
		fi
	fi
done

# Debug
printf "Using SSL dirs:\nSSL base:\t%s\nCert dir:\t%s\nKey dir:\t%s\n\n" "${SSL_DIR}" "${CERT_DIR}" "${KEY_DIR}"

# Do we need to create a CA?
if [ ! -f "${CERT_DIR}/ca.crt" ]; then
	printf "%s\n" "Creating CA Certificate...."
	printf "%s\n" "cockroach cert create-ca --certs-dir=\"${CERT_DIR}\" --ca-key=\"${KEY_DIR}/ca.key\""
	cockroach cert create-ca --certs-dir="${CERT_DIR}" --ca-key="${KEY_DIR}/ca.key"
fi

# Root cert
if [ ! -f "${CERT_DIR}/client.root.crt" ]; then
	printf "%s\n" "Creating client root certificate...."
	printf "%s\n" "cockroach cert create-client root --certs-dir=\"${CERT_DIR}\" --ca-key=\"${KEY_DIR}/ca.key\""
	cockroach cert create-client root --certs-dir="${CERT_DIR}" --ca-key="${KEY_DIR}/ca.key"
fi

# Node cert?
if [ ! -f "${CERT_DIR}/node.crt" ]; then
	printf "%s\n" "Creating NODE certificate...."
	printf "%s\n" "cockroach cert create-node localhost cockroachdb --certs-dir=\"${CERT_DIR}\" --ca-key=\"${KEY_DIR}/ca.key\""
	cockroach cert create-node localhost cockroachdb --certs-dir="${CERT_DIR}" --ca-key="${KEY_DIR}/ca.key"
fi

exit 0
