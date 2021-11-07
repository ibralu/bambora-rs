# \ProfilesApi

All URIs are relative to *https://api.na.bambora.com/v1*

Method | HTTP request | Description
------------- | ------------- | -------------
[**profiles_post**](ProfilesApi.md#profiles_post) | **POST** /profiles | Create Profile
[**profiles_profile_id_cards_card_id_delete**](ProfilesApi.md#profiles_profile_id_cards_card_id_delete) | **DELETE** /profiles/{profileId}/cards/{cardId} | Delete card
[**profiles_profile_id_cards_card_id_put**](ProfilesApi.md#profiles_profile_id_cards_card_id_put) | **PUT** /profiles/{profileId}/cards/{cardId} | Update card
[**profiles_profile_id_cards_get**](ProfilesApi.md#profiles_profile_id_cards_get) | **GET** /profiles/{profileId}/cards | Get cards
[**profiles_profile_id_cards_post**](ProfilesApi.md#profiles_profile_id_cards_post) | **POST** /profiles/{profileId}/cards | Add card
[**profiles_profile_id_delete**](ProfilesApi.md#profiles_profile_id_delete) | **DELETE** /profiles/{profileId} | Delete profile
[**profiles_profile_id_get**](ProfilesApi.md#profiles_profile_id_get) | **GET** /profiles/{profileId} | Get profile
[**profiles_profile_id_put**](ProfilesApi.md#profiles_profile_id_put) | **PUT** /profiles/{profileId} | Update Profile



## profiles_post

> crate::models::ProfileResponse profiles_post(create_profile_body)
Create Profile

Create a new Payment Profile using either a card or a Legato token. You must supply one of them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**create_profile_body** | [**CreateProfileBody**](CreateProfileBody.md) |  | [required] |

### Return type

[**crate::models::ProfileResponse**](ProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_profile_id_cards_card_id_delete

> crate::models::ProfileResponse profiles_profile_id_cards_card_id_delete(profile_id, card_id)
Delete card

Delete a card on the profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id. (aka CustomerCode) | [required] |
**card_id** | **i32** | The card id. | [required] |

### Return type

[**crate::models::ProfileResponse**](ProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_profile_id_cards_card_id_put

> crate::models::ProfileResponse profiles_profile_id_cards_card_id_put(profile_id, card_id, card)
Update card

Update the details of a card on the profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id. (aka CustomerCode) | [required] |
**card_id** | **i32** | The card id. | [required] |
**card** | [**ProfileCard**](ProfileCard.md) | The card that will be updated on the profile. | [required] |

### Return type

[**crate::models::ProfileResponse**](ProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_profile_id_cards_get

> crate::models::ProfileGetCards profiles_profile_id_cards_get(profile_id)
Get cards

Get all of the cards on the profile.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id. (aka CustomerCode) | [required] |

### Return type

[**crate::models::ProfileGetCards**](ProfileGetCards.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_profile_id_cards_post

> crate::models::ProfileResponse profiles_profile_id_cards_post(profile_id, card)
Add card

Add a card to the profile. Note that there is a default limit of 1 card per profile. You can change this in your Profiles settings in the back office.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id. (aka CustomerCode) | [required] |
**card** | [**ProfileCard**](ProfileCard.md) | The card that will be added to the profile. | [required] |

### Return type

[**crate::models::ProfileResponse**](ProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_profile_id_delete

> crate::models::ProfileResponse profiles_profile_id_delete(profile_id)
Delete profile

Delete a Payment Profile using the profile ID, also known as the Customer Code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id. (aka CustomerCode) | [required] |

### Return type

[**crate::models::ProfileResponse**](ProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_profile_id_get

> crate::models::PaymentProfile profiles_profile_id_get(profile_id)
Get profile

Get a Payment Profile using the profile ID, also known as the Customer Code.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id. (aka CustomerCode) | [required] |

### Return type

[**crate::models::PaymentProfile**](PaymentProfile.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## profiles_profile_id_put

> crate::models::ProfileResponse profiles_profile_id_put(profile_id, update_profile_body)
Update Profile

Create a new Payment Profile using either a card or a Legato token. You must supply one of them.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**profile_id** | **String** | The profile id. (aka CustomerCode) | [required] |
**update_profile_body** | [**UpdateProfileBody**](UpdateProfileBody.md) |  | [required] |

### Return type

[**crate::models::ProfileResponse**](ProfileResponse.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

