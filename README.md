# Arti Experiments

This is a small repo to learn more about [Arti](https://gitlab.torproject.org/tpo/core/arti), the experimental implementation of the Tor protocol written in Rust.

Arti provides an API to integrate Tor network connectivity into other programs. Being written in Rust, Arti has the unique advantage of providing both a secure (memory-safe, type-safe etc) and anonymous interface to the clearnet and Deep Web for a variety of applications

This repo is an exploration into that space, and since Arti is in an alpha state, doing so would probably uncover bugs and missing information in docs.

## Overview

This project aims to build a small download manager prototype which can connect to the Tor network and get a copy of the Tor browser from there.

## Setup

Make sure you have Rust installed and ready to go.

Clone this repo, and run ```cargo run```
to run the program. It will automatically fetch dependencies and compile the program.

Right now, it only downloads a specific Linux build of the Tor Browser Bundle as a file "download".
