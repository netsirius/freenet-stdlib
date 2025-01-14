// automatically generated by the FlatBuffers compiler, do not modify

/* eslint-disable @typescript-eslint/no-unused-vars, @typescript-eslint/no-explicit-any, @typescript-eslint/no-non-null-assertion */

import * as flatbuffers from 'flatbuffers';

import { DelegateType, unionToDelegateType, unionListToDelegateType } from '../client-request/delegate-type.js';
import { WasmDelegateV1, WasmDelegateV1T } from '../client-request/wasm-delegate-v1.js';


export class DelegateContainer implements flatbuffers.IUnpackableObject<DelegateContainerT> {
  bb: flatbuffers.ByteBuffer|null = null;
  bb_pos = 0;
  __init(i:number, bb:flatbuffers.ByteBuffer):DelegateContainer {
  this.bb_pos = i;
  this.bb = bb;
  return this;
}

static getRootAsDelegateContainer(bb:flatbuffers.ByteBuffer, obj?:DelegateContainer):DelegateContainer {
  return (obj || new DelegateContainer()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

static getSizePrefixedRootAsDelegateContainer(bb:flatbuffers.ByteBuffer, obj?:DelegateContainer):DelegateContainer {
  bb.setPosition(bb.position() + flatbuffers.SIZE_PREFIX_LENGTH);
  return (obj || new DelegateContainer()).__init(bb.readInt32(bb.position()) + bb.position(), bb);
}

delegateType():DelegateType {
  const offset = this.bb!.__offset(this.bb_pos, 4);
  return offset ? this.bb!.readUint8(this.bb_pos + offset) : DelegateType.NONE;
}

delegate<T extends flatbuffers.Table>(obj:any):any|null {
  const offset = this.bb!.__offset(this.bb_pos, 6);
  return offset ? this.bb!.__union(obj, this.bb_pos + offset) : null;
}

static startDelegateContainer(builder:flatbuffers.Builder) {
  builder.startObject(2);
}

static addDelegateType(builder:flatbuffers.Builder, delegateType:DelegateType) {
  builder.addFieldInt8(0, delegateType, DelegateType.NONE);
}

static addDelegate(builder:flatbuffers.Builder, delegateOffset:flatbuffers.Offset) {
  builder.addFieldOffset(1, delegateOffset, 0);
}

static endDelegateContainer(builder:flatbuffers.Builder):flatbuffers.Offset {
  const offset = builder.endObject();
  builder.requiredField(offset, 6) // delegate
  return offset;
}

static createDelegateContainer(builder:flatbuffers.Builder, delegateType:DelegateType, delegateOffset:flatbuffers.Offset):flatbuffers.Offset {
  DelegateContainer.startDelegateContainer(builder);
  DelegateContainer.addDelegateType(builder, delegateType);
  DelegateContainer.addDelegate(builder, delegateOffset);
  return DelegateContainer.endDelegateContainer(builder);
}

unpack(): DelegateContainerT {
  return new DelegateContainerT(
    this.delegateType(),
    (() => {
      const temp = unionToDelegateType(this.delegateType(), this.delegate.bind(this));
      if(temp === null) { return null; }
      return temp.unpack()
  })()
  );
}


unpackTo(_o: DelegateContainerT): void {
  _o.delegateType = this.delegateType();
  _o.delegate = (() => {
      const temp = unionToDelegateType(this.delegateType(), this.delegate.bind(this));
      if(temp === null) { return null; }
      return temp.unpack()
  })();
}
}

export class DelegateContainerT implements flatbuffers.IGeneratedObject {
constructor(
  public delegateType: DelegateType = DelegateType.NONE,
  public delegate: WasmDelegateV1T|null = null
){}


pack(builder:flatbuffers.Builder): flatbuffers.Offset {
  const delegate = builder.createObjectOffset(this.delegate);

  return DelegateContainer.createDelegateContainer(builder,
    this.delegateType,
    delegate
  );
}
}
