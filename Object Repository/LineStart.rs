<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>LineStart</name>
   <tag></tag>
   <elementGuidId>2781fadc-82d2-4e6d-9f29-61bc745dd89b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;\u003cLineStatus xmlns\u003d\&quot;http://schemas.datacontract.org/2004/07/InterfaceWCF.DataContracts.Workshop\&quot;\u003e\n  \u003cLine\u003ePB UATS2D\u003c/Line\u003e\n  \u003cStatuses\u003e\n    \u003cStatusDetails\u003e\n      \u003cWONum\u003e283\u003c/WONum\u003e\n      \u003cStatus\u003eSTART\u003c/Status\u003e\n      \u003cStatusDate\u003e2018-10-12 02:30:00\u003c/StatusDate\u003e\n      \u003cReasonCode\u003e\u003c/ReasonCode\u003e\n    \u003c/StatusDetails\u003e\n\n  \u003c/Statuses\u003e\n\u003c/LineStatus\u003e &quot;,
  &quot;contentType&quot;: &quot;application/xml&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>application/xml</value>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Basic Z3hvYXV0aDokR3hvMjAxN0F1dGg=</value>
   </httpHeaderProperties>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>http://10.155.43.156/GXOUpgrade_WCF_TEST/Services/Workshop/WorkorderSVC.svc/WORESTService/SaveLineStatus?</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceFunction></soapServiceFunction>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()



</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
