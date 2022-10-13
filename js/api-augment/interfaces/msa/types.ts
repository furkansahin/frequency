// Auto-generated via `yarn polkadot-types-from-defs`, do not edit
/* eslint-disable */

import type { Struct, u64 } from '@polkadot/types-codec';
import type { AccountId } from '@polkadot/types/interfaces/runtime';

/** @name KeyInfoResponse */
export interface KeyInfoResponse extends Struct {
  readonly key: AccountId;
  readonly msaId: MessageSourceId;
}

/** @name MessageSourceId */
export interface MessageSourceId extends u64 {}

export type PHANTOM_MSA = 'msa';