// Copyright 2017 Peter Williams <peter@newton.cx> and collaborators
// Licensed under the MIT License.

extern crate gcc;

const FILES: &[&str] = &[
    "casacore/casa/Arrays/Array2.cc",
    "casacore/casa/Arrays/Array2Math.cc",
    "casacore/casa/Arrays/ArrayBase.cc",
    "casacore/casa/Arrays/ArrayError.cc",
    "casacore/casa/Arrays/ArrayOpsDiffShapes.cc",
    "casacore/casa/Arrays/ArrayPartMath.cc",
    "casacore/casa/Arrays/ArrayPosIter.cc",
    "casacore/casa/Arrays/Array_tmpl.cc",
    "casacore/casa/Arrays/ArrayUtil2.cc",
    "casacore/casa/Arrays/AxesMapping.cc",
    "casacore/casa/Arrays/AxesSpecifier.cc",
    "casacore/casa/Arrays/ExtendSpecifier.cc",
    "casacore/casa/Arrays/IPosition2.cc",
    "casacore/casa/Arrays/IPosition.cc",
    "casacore/casa/Arrays/MaskArrMath2.cc",
    "casacore/casa/Arrays/Matrix2Math.cc",
    "casacore/casa/Arrays/Matrix_tmpl.cc",
    "casacore/casa/Arrays/Slice.cc",
    "casacore/casa/Arrays/Slicer.cc",
    "casacore/casa/Arrays/Vector_tmpl.cc",
    "casacore/casa/BasicMath/Math.cc",
    "casacore/casa/BasicMath/Primes.cc",
    "casacore/casa/BasicMath/Random.cc",
    "casacore/casa/BasicSL/Complex.cc",
    "casacore/casa/BasicSL/Constants.cc",
    "casacore/casa/BasicSL/IComplex.cc",
    "casacore/casa/BasicSL/RegexBase.cc",
    "casacore/casa/BasicSL/STLMath.cc",
    "casacore/casa/BasicSL/String.cc",
    "casacore/casa/Containers/Allocator.cc",
    "casacore/casa/Containers/Block.cc",
    "casacore/casa/Containers/Block_tmpl.cc",
    "casacore/casa/Containers/HashMap2.cc",
    "casacore/casa/Containers/IterError.cc",
    "casacore/casa/Containers/List2.cc",
    "casacore/casa/Containers/Map2.cc",
    "casacore/casa/Containers/Record2.cc",
    "casacore/casa/Containers/Record2Interface.cc",
    "casacore/casa/Containers/Record.cc",
    "casacore/casa/Containers/RecordDesc.cc",
    "casacore/casa/Containers/RecordDescRep.cc",
    "casacore/casa/Containers/RecordField2Writer.cc",
    "casacore/casa/Containers/RecordFieldId.cc",
    "casacore/casa/Containers/RecordInterface.cc",
    "casacore/casa/Containers/RecordRep.cc",
    "casacore/casa/Containers/Stack2.cc",
    "casacore/casa/Containers/StackError.cc",
    "casacore/casa/Containers/ValueHolder.cc",
    "casacore/casa/Containers/ValueHolderRep.cc",
    "casacore/casa/Exceptions/CasaErrorTools.cc",
    "casacore/casa/Exceptions/Error2.cc",
    "casacore/casa/HDF5/HDF5DataSet.cc",
    "casacore/casa/HDF5/HDF5DataType.cc",
    "casacore/casa/HDF5/HDF5Error.cc",
    "casacore/casa/HDF5/HDF5File.cc",
    "casacore/casa/HDF5/HDF5Group.cc",
    "casacore/casa/HDF5/HDF5HidMeta.cc",
    "casacore/casa/HDF5/HDF5Object.cc",
    "casacore/casa/HDF5/HDF5Record.cc",
    "casacore/casa/IO/AipsIO.cc",
    "casacore/casa/IO/BaseSinkSource.cc",
    "casacore/casa/IO/BucketBase.cc",
    "casacore/casa/IO/BucketBuffered.cc",
    "casacore/casa/IO/BucketCache.cc",
    "casacore/casa/IO/BucketFile.cc",
    "casacore/casa/IO/BucketMapped.cc",
    "casacore/casa/IO/ByteIO.cc",
    "casacore/casa/IO/ByteSink.cc",
    "casacore/casa/IO/ByteSinkSource.cc",
    "casacore/casa/IO/ByteSource.cc",
    "casacore/casa/IO/CanonicalIO.cc",
    "casacore/casa/IO/ConversionIO.cc",
    "casacore/casa/IO/FilebufIO.cc",
    "casacore/casa/IO/FiledesIO.cc",
    "casacore/casa/IO/FileLocker.cc",
    "casacore/casa/IO/LECanonicalIO.cc",
    "casacore/casa/IO/LockFile.cc",
    "casacore/casa/IO/MemoryIO.cc",
    "casacore/casa/IO/MFFileIO.cc",
    "casacore/casa/IO/MMapfdIO.cc",
    "casacore/casa/IO/MMapIO.cc",
    "casacore/casa/IO/MultiFileBase.cc",
    "casacore/casa/IO/MultiFile.cc",
    "casacore/casa/IO/MultiHDF5.cc",
    "casacore/casa/IO/RawIO.cc",
    "casacore/casa/IO/RegularFileIO.cc",
    "casacore/casa/IO/StreamIO.cc",
    "casacore/casa/IO/TapeIO.cc",
    "casacore/casa/IO/TypeIO.cc",
    "casacore/casa/Logging/LogFilter.cc",
    "casacore/casa/Logging/LogFilterInterface.cc",
    "casacore/casa/Logging/LogIO.cc",
    "casacore/casa/Logging/LogMessage.cc",
    "casacore/casa/Logging/LogOrigin.cc",
    "casacore/casa/Logging/LogSink.cc",
    "casacore/casa/Logging/LogSinkInterface.cc",
    "casacore/casa/Logging/MemoryLogSink.cc",
    "casacore/casa/Logging/NullLogSink.cc",
    "casacore/casa/Logging/StreamLogSink.cc",
    "casacore/casa/OS/CanonicalConversion.cc",
    "casacore/casa/OS/CanonicalDataConversion.cc",
    "casacore/casa/OS/Conversion.cc",
    "casacore/casa/OS/DataConversion.cc",
    "casacore/casa/OS/Directory.cc",
    "casacore/casa/OS/DirectoryIterator.cc",
    "casacore/casa/OS/DOos.cc",
    "casacore/casa/OS/DynLib.cc",
    "casacore/casa/OS/EnvVar.cc",
    "casacore/casa/OS/File.cc",
    "casacore/casa/OS/HostInfo.cc",
    "casacore/casa/OS/IBMConversion.cc",
    "casacore/casa/OS/IBMDataConversion.cc",
    "casacore/casa/OS/LECanonicalConversion.cc",
    "casacore/casa/OS/LECanonicalDataConversion.cc",
    "casacore/casa/OS/LittleEndianConversion.cc",
    "casacore/casa/OS/malloc.cc",
    "casacore/casa/OS/Memory.cc",
    "casacore/casa/OS/MemoryTrace.cc",
    "casacore/casa/OS/ModcompConversion.cc",
    "casacore/casa/OS/ModcompDataConversion.cc",
    "casacore/casa/OS/Mutex.cc",
    "casacore/casa/OS/Path.cc",
    "casacore/casa/OS/PrecTimer.cc",
    "casacore/casa/OS/RawDataConversion.cc",
    "casacore/casa/OS/RegularFile.cc",
    "casacore/casa/OS/SymLink.cc",
    "casacore/casa/OS/Time.cc",
    "casacore/casa/OS/Timer.cc",
    "casacore/casa/OS/VAXConversion.cc",
    "casacore/casa/OS/VAXDataConversion.cc",
    "casacore/casa/Quanta/Euler.cc",
    "casacore/casa/Quanta/MeasValue.cc",
    "casacore/casa/Quanta/MVAngle.cc",
    "casacore/casa/Quanta/MVBaseline.cc",
    "casacore/casa/Quanta/MVDirection.cc",
    "casacore/casa/Quanta/MVDoppler.cc",
    "casacore/casa/Quanta/MVDouble.cc",
    "casacore/casa/Quanta/MVEarthMagnetic.cc",
    "casacore/casa/Quanta/MVEpoch.cc",
    "casacore/casa/Quanta/MVFrequency.cc",
    "casacore/casa/Quanta/MVPosition.cc",
    "casacore/casa/Quanta/MVRadialVelocity.cc",
    "casacore/casa/Quanta/MVTime.cc",
    "casacore/casa/Quanta/MVuvw.cc",
    "casacore/casa/Quanta/QBase.cc",
    "casacore/casa/Quanta/QC.cc",
    "casacore/casa/Quanta/QLogical2.cc",
    "casacore/casa/Quanta/QMath2.cc",
    "casacore/casa/Quanta/Quantum2.cc",
    "casacore/casa/Quanta/QuantumHolder.cc",
    "casacore/casa/Quanta/RotMatrix.cc",
    "casacore/casa/Quanta/Unit.cc",
    "casacore/casa/Quanta/UnitDim.cc",
    "casacore/casa/Quanta/UnitMap2.cc",
    "casacore/casa/Quanta/UnitMap3.cc",
    "casacore/casa/Quanta/UnitMap4.cc",
    "casacore/casa/Quanta/UnitMap5.cc",
    "casacore/casa/Quanta/UnitMap6.cc",
    "casacore/casa/Quanta/UnitMap7.cc",
    "casacore/casa/Quanta/UnitMap.cc",
    "casacore/casa/Quanta/UnitName.cc",
    "casacore/casa/Quanta/UnitVal.cc",
    "casacore/casa/System/AipsrcBool.cc",
    "casacore/casa/System/Aipsrc.cc",
    "casacore/casa/System/AipsrcValue2.cc",
    "casacore/casa/System/AipsrcVBool.cc",
    "casacore/casa/System/AipsrcVString.cc",
    "casacore/casa/System/AppInfo.cc",
    "casacore/casa/System/Casarc.cc",
    "casacore/casa/System/Choice.cc",
    "casacore/casa/System/ObjectID2.cc",
    "casacore/casa/System/ObjectID.cc",
    "casacore/casa/System/PGPlotter.cc",
    "casacore/casa/System/PGPlotterInterface.cc",
    "casacore/casa/System/PGPlotterNull.cc",
    "casacore/casa/System/ProgressMeter.cc",
    "casacore/casa/Utilities/AlignMemory.cc",
    "casacore/casa/Utilities/BitVector.cc",
    "casacore/casa/Utilities/Compare.cc",
    "casacore/casa/Utilities/CompositeNumber.cc",
    "casacore/casa/Utilities/Copy2.cc",
    "casacore/casa/Utilities/CountedPtr2.cc",
    "casacore/casa/Utilities/cregex.cc",
    "casacore/casa/Utilities/DataType.cc",
    "casacore/casa/Utilities/DynBuffer.cc",
    "casacore/casa/Utilities/Fallible2.cc",
    "casacore/casa/Utilities/MUString.cc",
    "casacore/casa/Utilities/Notice.cc",
    "casacore/casa/Utilities/Precision.cc",
    "casacore/casa/Utilities/RecordTransformable.cc",
    "casacore/casa/Utilities/Regex.cc",
    "casacore/casa/Utilities/RegSequence.cc",
    "casacore/casa/Utilities/Sequence2.cc",
    "casacore/casa/Utilities/Sort.cc",
    "casacore/casa/Utilities/SortError.cc",
    "casacore/casa/Utilities/StringDistance.cc",
    "casacore/casa/Utilities/ValType.cc",
    "casacore/tables/DataMan/BitFlagsEngine.cc",
    "casacore/tables/DataMan/CompressComplex.cc",
    "casacore/tables/DataMan/CompressFloat.cc",
    "casacore/tables/DataMan/DataManAccessor.cc",
    "casacore/tables/DataMan/DataManager.cc",
    "casacore/tables/DataMan/DataManError.cc",
    "casacore/tables/DataMan/DataManInfo.cc",
    "casacore/tables/DataMan/ForwardCol.cc",
    "casacore/tables/DataMan/ForwardColRow.cc",
    "casacore/tables/DataMan/IncrementalStMan.cc",
    "casacore/tables/DataMan/IncrStManAccessor.cc",
    "casacore/tables/DataMan/ISMBase.cc",
    "casacore/tables/DataMan/ISMBucket.cc",
    "casacore/tables/DataMan/ISMColumn.cc",
    "casacore/tables/DataMan/ISMIndColumn.cc",
    "casacore/tables/DataMan/ISMIndex.cc",
    "casacore/tables/DataMan/MemoryStMan.cc",
    "casacore/tables/DataMan/MSMBase.cc",
    "casacore/tables/DataMan/MSMColumn.cc",
    "casacore/tables/DataMan/MSMDirColumn.cc",
    "casacore/tables/DataMan/MSMIndColumn.cc",
    "casacore/tables/DataMan/SSMBase.cc",
    "casacore/tables/DataMan/SSMColumn.cc",
    "casacore/tables/DataMan/SSMDirColumn.cc",
    "casacore/tables/DataMan/SSMIndColumn.cc",
    "casacore/tables/DataMan/SSMIndex.cc",
    "casacore/tables/DataMan/SSMIndStringColumn.cc",
    "casacore/tables/DataMan/SSMStringHandler.cc",
    "casacore/tables/DataMan/StandardStManAccessor.cc",
    "casacore/tables/DataMan/StandardStMan.cc",
    "casacore/tables/DataMan/StArrAipsIO.cc",
    "casacore/tables/DataMan/StArrayFile.cc",
    "casacore/tables/DataMan/StIndArrAIO.cc",
    "casacore/tables/DataMan/StIndArray.cc",
    "casacore/tables/DataMan/StManAipsIO.cc",
    "casacore/tables/DataMan/StManColumn.cc",
    "casacore/tables/DataMan/TiledCellStMan.cc",
    "casacore/tables/DataMan/TiledColumnStMan.cc",
    "casacore/tables/DataMan/TiledDataStManAccessor.cc",
    "casacore/tables/DataMan/TiledDataStMan.cc",
    "casacore/tables/DataMan/TiledFileAccess.cc",
    "casacore/tables/DataMan/TiledFileHelper.cc",
    "casacore/tables/DataMan/TiledShapeStMan.cc",
    "casacore/tables/DataMan/TiledStManAccessor.cc",
    "casacore/tables/DataMan/TiledStMan.cc",
    "casacore/tables/DataMan/TSMColumn.cc",
    "casacore/tables/DataMan/TSMCoordColumn.cc",
    "casacore/tables/DataMan/TSMCubeBuff.cc",
    "casacore/tables/DataMan/TSMCube.cc",
    "casacore/tables/DataMan/TSMCubeMMap.cc",
    "casacore/tables/DataMan/TSMDataColumn.cc",
    "casacore/tables/DataMan/TSMFile.cc",
    "casacore/tables/DataMan/TSMIdColumn.cc",
    "casacore/tables/DataMan/TSMOption.cc",
    "casacore/tables/DataMan/TSMShape.cc",
    "casacore/tables/DataMan/VirtColEng.cc",
    "casacore/tables/DataMan/VirtualTaQLColumn.cc",
    "casacore/tables/Tables/ArrayColumn_tmpl.cc",
    "casacore/tables/Tables/ArrColDesc_tmpl.cc",
    "casacore/tables/Tables/BaseColDesc.cc",
    "casacore/tables/Tables/BaseColumn.cc",
    "casacore/tables/Tables/BaseTabIter.cc",
    "casacore/tables/Tables/BaseTable.cc",
    "casacore/tables/Tables/ColDescSet.cc",
    "casacore/tables/Tables/ColumnCache.cc",
    "casacore/tables/Tables/ColumnDesc.cc",
    "casacore/tables/Tables/ColumnSet.cc",
    "casacore/tables/Tables/ColumnsIndexArray.cc",
    "casacore/tables/Tables/ColumnsIndex.cc",
    "casacore/tables/Tables/ConcatColumn.cc",
    "casacore/tables/Tables/ConcatRows.cc",
    "casacore/tables/Tables/ConcatTable.cc",
    "casacore/tables/Tables/ExternalLockSync.cc",
    "casacore/tables/Tables/MemoryTable.cc",
    "casacore/tables/Tables/NullTable.cc",
    "casacore/tables/Tables/PlainColumn.cc",
    "casacore/tables/Tables/PlainTable.cc",
    "casacore/tables/Tables/ReadAsciiTable.cc",
    "casacore/tables/Tables/RefColumn.cc",
    "casacore/tables/Tables/RefRows.cc",
    "casacore/tables/Tables/RefTable.cc",
    "casacore/tables/Tables/RowCopier.cc",
    "casacore/tables/Tables/ScaColDesc_tmpl.cc",
    "casacore/tables/Tables/ScalarColumn_tmpl.cc",
    "casacore/tables/Tables/ScaRecordColData.cc",
    "casacore/tables/Tables/ScaRecordColDesc.cc",
    "casacore/tables/Tables/SetupNewTab.cc",
    "casacore/tables/Tables/StorageOption.cc",
    "casacore/tables/Tables/SubTabDesc.cc",
    "casacore/tables/Tables/TableAttr.cc",
    "casacore/tables/Tables/TableCache.cc",
    "casacore/tables/Tables/Table.cc",
    "casacore/tables/Tables/TableColumn.cc",
    "casacore/tables/Tables/TableCopy.cc",
    "casacore/tables/Tables/TableDesc.cc",
    "casacore/tables/Tables/TableError.cc",
    "casacore/tables/Tables/TableInfo.cc",
    "casacore/tables/Tables/TableIter.cc",
    "casacore/tables/Tables/TableKeyword.cc",
    "casacore/tables/Tables/TableLock.cc",
    "casacore/tables/Tables/TableLockData.cc",
    "casacore/tables/Tables/TableLocker.cc",
    "casacore/tables/Tables/TableRecord.cc",
    "casacore/tables/Tables/TableRecordRep.cc",
    "casacore/tables/Tables/TableRow.cc",
    "casacore/tables/Tables/TableSyncData.cc",
    "casacore/tables/Tables/TableTrace.cc",
    "casacore/tables/Tables/TabPath.cc",
    "casacore/tables/TaQL/ExprAggrNodeArray.cc",
    "casacore/tables/TaQL/ExprAggrNode.cc",
    "casacore/tables/TaQL/ExprConeNode.cc",
    "casacore/tables/TaQL/ExprDerNodeArray.cc",
    "casacore/tables/TaQL/ExprDerNode.cc",
    "casacore/tables/TaQL/ExprFuncNodeArray.cc",
    "casacore/tables/TaQL/ExprFuncNode.cc",
    "casacore/tables/TaQL/ExprGroupAggrFuncArray.cc",
    "casacore/tables/TaQL/ExprGroupAggrFunc.cc",
    "casacore/tables/TaQL/ExprGroup.cc",
    "casacore/tables/TaQL/ExprLogicNodeArray.cc",
    "casacore/tables/TaQL/ExprLogicNode.cc",
    "casacore/tables/TaQL/ExprMathNodeArray.cc",
    "casacore/tables/TaQL/ExprMathNode.cc",
    "casacore/tables/TaQL/ExprNodeArray.cc",
    "casacore/tables/TaQL/ExprNode.cc",
    "casacore/tables/TaQL/ExprNodeRecord.cc",
    "casacore/tables/TaQL/ExprNodeRep.cc",
    "casacore/tables/TaQL/ExprNodeSet.cc",
    "casacore/tables/TaQL/ExprRange.cc",
    "casacore/tables/TaQL/ExprUDFNodeArray.cc",
    "casacore/tables/TaQL/ExprUDFNode.cc",
    "casacore/tables/TaQL/ExprUnitNode.cc",
    "casacore/tables/TaQL/MArrayBase.cc",
    "casacore/tables/TaQL/RecordExpr.cc",
    "casacore/tables/TaQL/RecordGram.cc",
    "casacore/tables/TaQL/TableExprData.cc",
    "casacore/tables/TaQL/TableExprId.cc",
    "casacore/tables/TaQL/TableGram.cc",
    "casacore/tables/TaQL/TableParse.cc",
    "casacore/tables/TaQL/TaQLNode.cc",
    "casacore/tables/TaQL/TaQLNodeDer.cc",
    "casacore/tables/TaQL/TaQLNodeHandler.cc",
    "casacore/tables/TaQL/TaQLNodeRep.cc",
    "casacore/tables/TaQL/TaQLNodeResult.cc",
    "casacore/tables/TaQL/TaQLNodeVisitor.cc",
    "casacore/tables/TaQL/TaQLResult.cc",
    "casacore/tables/TaQL/TaQLShow.cc",
    "casacore/tables/TaQL/TaQLStyle.cc",
    "casacore/tables/TaQL/UDFBase.cc",
];

fn main() {
    gcc::Build::new()
        .cpp(true)
        .warnings(true)
        .include(".")
        .files(FILES)
        .compile("libcasatables_impl.a");

    for file in FILES {
        println!("cargo:rerun-if-changed={}", file);
    }
}