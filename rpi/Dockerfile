ARG CROSS_BASE_IMAGE

FROM $CROSS_BASE_IMAGE

ARG CROSS_DEB_ARCH
ENV CROSS_DEB_ARCH=$CROSS_DEB_ARCH
ENV DEBIAN_FRONTEND=noninteractive

# Needed to install certain local dependencies
RUN apt-get update && \
    apt-get install --assume-yes --no-install-recommends \
    ca-certificates \
    curl \
    cpio \
    sharutils \
    gnupg

# Temporarily use debian sources rather than ubuntu.
RUN mv /etc/apt/sources.list /etc/apt/sources.list.bak && \
    echo "deb http://http.debian.net/debian/ bookworm main" > /etc/apt/sources.list && \
    echo "deb http://security.debian.org/ bookworm-security main" >> /etc/apt/sources.list

# Add architecture
RUN dpkg --add-architecture "${CROSS_DEB_ARCH}" || \
    echo "foreign-architecture ${CROSS_DEB_ARCH}" > /etc/dpkg/dpkg.cfg.d/multiarch

# Add Debian keys
RUN curl --retry 3 -sSfL 'https://ftp-master.debian.org/keys/archive-key-{7.0,8,9,10}.asc' -O && \
    curl --retry 3 -sSfL 'https://ftp-master.debian.org/keys/archive-key-{8,9,10}-security.asc' -O && \
    curl --retry 3 -sSfL 'https://ftp-master.debian.org/keys/release-{7,8,9,10}.asc' -O && \
    curl --retry 3 -sSfL 'https://www.ports.debian.org/archive_{2020,2021,2022}.key' -O && \
    for key in *.asc *.key; do apt-key add "${key}" && rm "${key}"; done

# Allow apt-get to retry downloads
RUN echo 'APT::Acquire::Retries "3";' > /etc/apt/apt.conf.d/80-retries

# Install required dependencies
RUN apt-get update && \
    apt-get install --assume-yes \
    libcec-dev:${CROSS_DEB_ARCH} \
    libp8-platform-dev:${CROSS_DEB_ARCH} \
    libudev-dev:${CROSS_DEB_ARCH} \
    libc6-dev:${CROSS_DEB_ARCH}

# Restore ubuntu sources
RUN  mv -f /etc/apt/sources.list.bak /etc/apt/sources.list && \
    if [ -f /etc/dpkg/dpkg.cfg.d/multiarch.bak ]; then mv /etc/dpkg/dpkg.cfg.d/multiarch.bak /etc/dpkg/dpkg.cfg.d/multiarch; fi

# Remove architecture
# Can fail if arch is used (amd64 and/or i386)
RUN dpkg --remove-architecture "${CROSS_DEB_ARCH}" || true
RUN apt-get update
