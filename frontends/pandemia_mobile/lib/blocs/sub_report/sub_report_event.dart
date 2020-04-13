import 'package:equatable/equatable.dart';
import 'package:meta/meta.dart';
import 'package:pandemia_mobile/models/sub_report.dart';

@immutable
abstract class SubReportEvent extends Equatable {
  SubReportEvent([List props = const []]) : super(props);
}

class LoadSubReport extends SubReportEvent {
  final bool force;
  final int status;
  final bool withLoading;
  LoadSubReport({this.status, this.force = false, this.withLoading = true});

  @override
  String toString() => "LoadSubReport";
}

class CreateSubReport extends SubReportEvent {
  final String fullName;
  final int age;
  final String residenceAddress;
  final String gender;
  final String arrivalAddress;
  final String arrivalDate;
  final String desc;
  final int status;
  final List<String> complaint;

  CreateSubReport(
      this.fullName,
      this.age,
      this.residenceAddress,
      this.gender,
      this.arrivalAddress,
      this.arrivalDate,
      this.desc,
      this.status,
      this.complaint);
  @override
  String toString() => "CreateSubReport";
}

class UpdateSubReport extends SubReportEvent {
  final int id;
  final String fullName;
  final int age;
  final String residenceAddress;
  final String gender;
  final String arrivalAddress;
  final String arrivalDate;
  final String desc;
  final int status;
  final List<String> complaint;

  UpdateSubReport(
      this.id,
      this.fullName,
      this.age,
      this.residenceAddress,
      this.gender,
      this.arrivalAddress,
      this.arrivalDate,
      this.desc,
      this.status,
      this.complaint);
  @override
  String toString() => "UpdateSubReport";
}

/// Event to delete SubReport
class DeleteSubReport extends SubReportEvent {
  final SubReport subReport;
  DeleteSubReport(this.subReport);
  @override
  String toString() => "DeleteSubReport";
}

class SubReportSearch extends SubReportEvent {
  final String query;
  final int status;
  SubReportSearch(this.query, this.status);
  @override
  String toString() => "SubReportSearch";
}
