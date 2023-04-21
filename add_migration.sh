#!/bin/bash

touch migrations/$(date +%Y%m%d%H%M)_$1.sql

code .
